extern crate cargo;
#[macro_use]
extern crate nom;
extern crate rustc_serialize;

use std::cmp;
use std::collections::BTreeMap;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::{self, File};
use std::str;
use std::io::Write;
use std::io::Read;
use std::io;
use std::path::Path;
use rustc_serialize::hex::ToHex;

use cargo::ops::Packages;
use cargo::core::{PackageId, SourceId, Dependency, Workspace};
use cargo::ops;
use cargo::CliResult;
use cargo::util::{human, CliError, ChainError, ToUrl, Config};
use cargo::util::Sha256;

// Sidestep format!'s "string literals only" stipulation
macro_rules! BUILD_FILE_TEMPLATE {() => (r#"package(default_visibility = ["//visibility:public"])

licenses(["notice"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)
rust_library(
    name = "{sanitized_crate_name}",
    srcs = glob(["lib.rs", "src/**/*.rs"]),
    deps = [
{comma_separated_cargo_deps}    ],
    rustc_flags = [
        "--cap-lints warn",
    ],
    crate_features = [
{comma_separated_features}    ],
)"#)}

macro_rules! NEW_HTTP_ARCHIVE_TEMPLATE {() => (r#"new_http_archive(
    name = "io_crates_{sanitized_crate_name}_{sanitized_crate_version}",
    urls = [
      # Bazel's downloader renders HTTP page instead of downloading for some reason.
      #"https://crates.io/api/v1/crates/{crate_name}/{crate_version}/download"
      "https://crates-io.s3-us-west-1.amazonaws.com/crates/{crate_name}/{crate_name}-{crate_version}.crate",
    ],
    type = "tar.gz",
    strip_prefix = "{crate_name}-{crate_version}",
    build_file_content = """
{build_file_content}
""",
)
"#)}

macro_rules! VENDOR_DIR_WORKSPACE_SNIPPET {() => (r#"
local_repository(
    name = "{name}",
    path = __workspace_dir__ + "{path}"
)"#)}


/** Define parser for --overrides flag */
fn isnt_plus(chr: char) -> bool { chr != '+' }
fn isnt_colon(chr: char) -> bool { chr != ':' }
fn isnt_comma(chr: char) -> bool { chr != ',' }
named!(parse_override( &str ) -> DependencyOverride,
   do_parse!(
     name: map!(take_while_s!(isnt_plus), str::to_owned) >>
     char!('+') >>
     version: map!(take_while_s!(isnt_colon), str::to_owned) >>
     char!(':') >>
     bazel_path: map!(take_while_s!(isnt_comma), str::to_owned) >>
     (DependencyOverride { name: name, version: version, bazel_path: bazel_path })
   )
);
named!(parse_overrides( &str ) -> Vec<DependencyOverride>, separated_list!(char!(','), parse_override));


#[derive(RustcDecodable)]
struct Options {
    flag_sync: Option<String>,
    flag_host: Option<String>,
    flag_verbose: u32,
    flag_generate: Option<bool>,
    flag_vendor: Option<String>,
    flag_quiet: Option<bool>,
    flag_color: Option<String>,
    flag_overrides: Option<String>
}

#[derive(Debug)]
struct DependencyOverride {
    pub name: String,
    pub version: String,
    pub bazel_path: String,
}

#[derive(Debug)]
struct RazePackage {
  pub id: PackageId,
  pub features: HashSet<String>,
  pub dependencies: Vec<Dependency>,
}

fn main() {
    // TODO(acmcarther): This will break soon. The method call is now "call_main_without_stdin"
    cargo::execute_main_without_stdin(real_main, false, r#"
Generate a Bazel WORKSPACE consisting of resolved dependencies for the current platform

Usage:
    cargo raze [options]

Options:
    -h, --help               Print this message
    -s, --sync LOCK          Sync the registry with LOCK
    --host HOST              Registry index to sync with
    -v, --verbose            Use verbose output
    -q, --quiet              No output printed to stdout
    --color WHEN             Coloring: auto, always, never
    -g, --generate           Generate a series of WORKSPACE rules to be included manually
    --vendor WHERE           Pull sources and generate BUILD files locally
    --overrides LIST         Comma separated cargo dependency overrides ["libc+0.2.21:@workspace//path:dep,..."]
"#)
}

fn real_main(options: Options, config: &Config) -> CliResult<Option<()>> {
    try!(config.configure(options.flag_verbose,
                          options.flag_quiet,
                          &options.flag_color,
                          /* frozen = */ false,
                          /* locked = */ false));

    let vendor_destination = options.flag_vendor.as_ref().map(|p| Path::new(p));
    let generate_workspace_rules = options.flag_generate.unwrap_or(false);
    if vendor_destination.is_none() && !generate_workspace_rules {
      println!("Not generating vendored dependencies or workspace rules; exiting.");
      return Ok(None);
    }

    if vendor_destination.is_some() && !vendor_destination.unwrap().starts_with("./") {
      println!("Please specify vendor destination as a relative path starting with './'");
      return Ok(None);
    }

    let registry_id = try!(load_registry(options.flag_host, &config));
    let lockfile_path = try!(find_lock_file_path(options.flag_sync));
    let lockfile = Path::new(&lockfile_path);
    let manifest_path = lockfile.parent().unwrap().join("Cargo.toml");
    let manifest = env::current_dir().unwrap().join(&manifest_path);
    let mut registry = registry_id.load(config);
    let ws = try!(Workspace::new(&manifest, config));
    let specs = Packages::All.into_package_id_specs(&ws).chain_error(|| {
      human("failed to find specs? whats a spec?")
    })?;

    // TODO(acmcarther): Fix unwrap. I'm unwrapping here temporarily because Nom's err is hard to
    // convert to CargoError
    let overrides = options.flag_overrides.as_ref()
      .map(|f| parse_overrides(f).to_result().unwrap())
      .unwrap_or(Vec::new());
    let override_name_and_ver_to_path: HashMap<(String, String), String> = overrides.into_iter()
      .map(|entry| ((entry.name, entry.version), entry.bazel_path))
      .collect();

    let (_, resolve) = ops::resolve_ws_precisely(
            &ws,
            None,
            &[],
            false,
            false,
            &specs).chain_error(|| {
        human("failed to load pkg lockfile")
    })?;

    let hash = cargo::util::hex::short_hash(&registry_id);
    let ident = registry_id.url().host().unwrap().to_string();
    let part = format!("{}-{}", ident, hash);
    let src = config.registry_source_path().join(&part);

    let mut package_ids = resolve.iter()
                     .filter(|id| *id.source_id() == registry_id)
                     .cloned()
                     .collect::<Vec<_>>();
    package_ids.sort_by_key(|id| id.name().to_owned());

    let mut max = HashMap::new();
    for id in package_ids.iter() {
        let max = max.entry(id.name()).or_insert(id.version());
        *max = cmp::max(id.version(), *max)
    }

    if vendor_destination.is_some() {
      let destination = vendor_destination.unwrap();
      let _ = fs::create_dir(&destination);
    }
    let mut bazel_workspace_entries = Vec::new();

    let mut raze_packages = Vec::new();

    for id in package_ids.iter() {
        let vers = format!("={}", id.version());
        let dep = try!(Dependency::parse_no_deprecated(id.name(),
                                                       Some(&vers[..]),
                                                       id.source_id()));
        {
          let mut vec = try!(registry.query(&dep));

          // Some versions have "build metadata" which is ignored by semver when
          // matching. That means that `vec` being returned may have more than one
          // element, so we filter out all non-equivalent versions with different
          // build metadata than the one we're looking for.
          //
          // Note that we also don't compare semver versions directly as the
          // default equality ignores build metadata.
          if vec.len() > 1 {
              vec.retain(|version| {
                  version.package_id().version().to_string() == id.version().to_string()
              });
          }
          if vec.len() == 0 {
              return Err(CliError::new(human(format!("could not find package: {}", id)), 101))
          }
          if vec.len() > 1 {
              return Err(CliError::new(human(format!("found too many packages: {}", id)), 101))
          }
        }

        // Skip generating new_crate_repository for overrides
        if override_name_and_ver_to_path.contains_key(&(id.name().to_owned(), id.version().to_string())) {
          continue
        }

        raze_packages.push(RazePackage {
          id: id.clone(),
          features: resolve.features(id)
            .cloned()
            .unwrap_or(HashSet::new()),
          dependencies: Vec::new()
        });

        // TODO(acmcarther): This will break as of cargo commit 50f1c172
        let mut feature_strs = resolve.features(id)
          .cloned()
          .unwrap_or(HashSet::new())
          .iter()
          .map(|f| format!("        \"{}\",\n", f))
          .collect::<Vec<String>>();
        feature_strs.sort();

        let feature_str = feature_strs.into_iter().collect::<String>();

        if generate_workspace_rules {
            // TODO(acmcarther): Filter build_dependencies out of this list, and emit a warning.
            let mut dep_strs = resolve.deps(id).into_iter()
              .map(|dep| {
                if override_name_and_ver_to_path.contains_key(&(dep.name().to_owned(), dep.version().to_string())) {
                  format!("        \"{}\",\n", override_name_and_ver_to_path.get(&(dep.name().to_owned(), dep.version().to_string())).unwrap())
                } else {
                  format!("        \"@io_crates_{sanitized_name}_{sanitized_crate_version}//:{sanitized_name}\",\n",
                          sanitized_name=dep.name().replace("-", "_"),
                          sanitized_crate_version=dep.version().to_string().replace(".", "_"))
                }
              })
              .collect::<Vec<String>>();
            dep_strs.sort();
            let dep_str = dep_strs.into_iter().collect::<String>();

            let build_file_content = format!(BUILD_FILE_TEMPLATE!(),
                                      sanitized_crate_name=id.name().replace("-", "_"),
                                      comma_separated_cargo_deps=dep_str,
                                      comma_separated_features=feature_str);

            let new_http_archive_decl = format!(NEW_HTTP_ARCHIVE_TEMPLATE!(),
                                      crate_name=id.name(),
                                      sanitized_crate_name=id.name().replace("-", "_"),
                                      crate_version=id.version(),
                                      sanitized_crate_version=id.version().to_string().replace(".", "_"),
                                      build_file_content=build_file_content);
            bazel_workspace_entries.push(new_http_archive_decl)
        }

        if vendor_destination.is_some() {
            let destination = vendor_destination.unwrap();

            // TODO(acmcarther): handle this properly
            registry.download(id).unwrap();

            // Next up, copy it to the vendor directory
            let name = format!("{}-{}", id.name(), id.version());
            let src = src.join(&name).into_path_unlocked();
            let dst_name = format!("{}-{}", id.name().replace("-","_"), id.version());
            let dst = destination.join(&dst_name);
            let _ = fs::remove_dir_all(&dst);
            let mut map = BTreeMap::new();
            try!(cp_r(&src, &dst, &dst, &mut map).chain_error(|| {
                human(format!("failed to copy over vendored sources for: {}", id))
            }));

            // TODO(acmcarther): Filter build_dependencies out of this list, and emit a warning.
            let mut dep_strs = resolve.deps(id).into_iter()
              .map(|dep| {
                if override_name_and_ver_to_path.contains_key(&(dep.name().to_owned(), dep.version().to_string())) {
                  format!("        \"{}\",\n", override_name_and_ver_to_path.get(&(dep.name().to_owned(), dep.version().to_string())).unwrap())
                } else {
                  format!("        \"//{crate_full_name}:{sanitized_name}\",\n",
                          crate_full_name=format!("{}-{}", dep.name().replace("-", "_"), dep.version()),
                          sanitized_name=dep.name().replace("-", "_"))
                }
              })
              .collect::<Vec<String>>();
            dep_strs.sort();
            let dep_str = dep_strs.into_iter().collect::<String>();
            let build_file_content = format!(BUILD_FILE_TEMPLATE!(),
                                      sanitized_crate_name=id.name().replace("-", "_"),
                                      comma_separated_cargo_deps=dep_str,
                                      comma_separated_features=feature_str);
            let build_file_path = dst.join("BUILD");
            try!(File::create(&build_file_path).and_then(|mut f| f.write_all(build_file_content.as_bytes())).chain_error(|| {
                human(format!("failed to create: `{}`", build_file_path.display()))
            }));
        }
    }

    println!("raze_packages: {:?}", raze_packages);

    if generate_workspace_rules {
        let bazel_workspace_content = bazel_workspace_entries.into_iter().collect::<String>();
        let workspace_path = Path::new("raze.WORKSPACE");
        try!(File::create(workspace_path).and_then(|mut f| f.write_all(bazel_workspace_content.as_bytes())).chain_error(|| {
            human(format!("failed to create: `{}`", workspace_path.display()))
        }));

        println!("--generate: A raze.WORKSPACE was created containing the generated dependencies. Integrate this into your existing WORKSPACE.");
    }

    if vendor_destination.is_some() {
        let workspace_path = vendor_destination.unwrap().join("WORKSPACE");
        try!(File::create(&workspace_path).chain_error(|| {
            human(format!("failed to create: `{}`", workspace_path.display()))
        }));

        println!("--vendor: Add the following snippet to your local workspace so bazel can find it: {}", format!(VENDOR_DIR_WORKSPACE_SNIPPET!(),
          name = "raze",
          path = vendor_destination.unwrap().to_string_lossy()[1..].to_owned()));
    }

    Ok(None)
}


fn load_registry(flag_host: Option<String>, config: &Config) -> CliResult<SourceId> {
    let source_id_from_registry =
      flag_host.map(|s| s.to_url().map(|url| SourceId::for_registry(&url)).map_err(human));

    source_id_from_registry.unwrap_or_else(|| SourceId::crates_io(config))
      .map_err(CliError::from)
}

fn find_lock_file_path(flag_sync: Option<String>) -> CliResult<String> {
    match flag_sync {
        Some(file) => Ok(file),
        None => {
            try!(fs::metadata("Cargo.lock").chain_error(|| {
                human("could not find `Cargo.lock`, must be run in a directory \
                       with Cargo.lock or use the `--sync` option")
            }));
            Ok("Cargo.lock".to_owned())
        }
    }
}

fn cp_r(src: &Path,
        dst: &Path,
        root: &Path,
        cksums: &mut BTreeMap<String, String>) -> io::Result<()> {
    try!(fs::create_dir(dst));
    for entry in try!(src.read_dir()) {
        let entry = try!(entry);

        // Skip .gitattributes as they're not relevant to builds most of the
        // time and if we respect them (e.g. in git) then it'll probably mess
        // with the checksums.
        if entry.file_name().to_str() == Some(".gitattributes") {
            continue
        }

        let src = entry.path();
        let dst = dst.join(entry.file_name());
        if try!(entry.file_type()).is_dir() {
            try!(cp_r(&src, &dst, root, cksums));
        } else {
            try!(fs::copy(&src, &dst));
            let rel = dst.strip_prefix(root).unwrap().to_str().unwrap();
            cksums.insert(rel.replace("\\", "/"), try!(sha256(&dst)));
        }
    }
    Ok(())
}

fn sha256(p: &Path) -> io::Result<String> {
    let mut file = try!(File::open(p));
    let mut sha = Sha256::new();
    let mut buf = [0; 2048];
    loop {
        let n = try!(file.read(&mut buf));
        if n == 0 {
            break
        }
        sha.update(&buf[..n]);
    }
    Ok(sha.finish().to_hex())
}
