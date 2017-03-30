extern crate cargo;
#[macro_use]
extern crate nom;
extern crate rustc_serialize;

use std::cmp;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::env;
use std::fs::{self, File};
use std::str;
use std::io::{self, Read, Write};
use std::path::Path;

use rustc_serialize::hex::ToHex;

use cargo::ops::Packages;
use cargo::core::{SourceId, Dependency, Workspace};
use cargo::ops;
use cargo::CliResult;
use cargo::util::{human, ChainError, ToUrl, Config, CargoResult};
use cargo::util::Sha256;


#[derive(RustcDecodable)]
struct Options {
    arg_path: Option<String>,
    flag_sync: Option<String>,
    flag_host: Option<String>,
    flag_verbose: u32,
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

fn main() {
    cargo::execute_main_without_stdin(real_main, false, r#"
Vendor all dependencies for a project locally

Usage:
    cargo raze vendor [options] [<path>]

Options:
    -h, --help               Print this message
    -s, --sync LOCK          Sync the registry with LOCK
    --host HOST              Registry index to sync with
    -v, --verbose            Use verbose output
    -q, --quiet              No output printed to stdout
    --color WHEN             Coloring: auto, always, never
    --overrides LIST         Comma separated cargo dependency overrides ["libc+0.2.21:@workspace//path:dep,..."]
"#)
}

fn real_main(options: Options, config: &Config) -> CliResult<Option<()>> {
    try!(config.configure(options.flag_verbose,
                          options.flag_quiet,
                          &options.flag_color,
                          /* frozen = */ false,
                          /* locked = */ false));

    let default = "vendor".to_string();
    let path = Path::new(options.arg_path.as_ref().unwrap_or(&default));

    let default_overrides = "".to_string();
    // TODO(acmcarther): fix below NO GOOD VERY BAD NO NO TIMES
    // Context: nom::ErrorKind isn't error_chain-able, so i'm unwrapping and walking away.
    let dependency_overrides = parse_overrides(&options.flag_overrides.as_ref().unwrap_or(&default_overrides)).to_result().unwrap();
    try!(fs::create_dir_all(&path).chain_error(|| {
        human(format!("failed to create: `{}`", path.display()))
    }));
    let id = try!(options.flag_host.map(|s| {
        s.to_url().map(|url| SourceId::for_registry(&url)).map_err(human)
    }).unwrap_or_else(|| {
        SourceId::crates_io(config)
    }));

    let lockfile = match options.flag_sync {
        Some(ref file) => file,
        None => {
            try!(fs::metadata("Cargo.lock").chain_error(|| {
                human("could not find `Cargo.lock`, must be run in a directory \
                       with Cargo.lock or use the `--sync` option")
            }));
            "Cargo.lock"
        }
    };

    try!(sync(Path::new(lockfile), &path, &id, config, dependency_overrides).chain_error(|| {
        human("failed to sync")
    }));

    Ok(None)
}

fn sync(lockfile: &Path,
        local_dst: &Path,
        registry_id: &SourceId,
        config: &Config,
        overrides: Vec<DependencyOverride>) -> CargoResult<()> {
    let mut registry = registry_id.load(config);
    let manifest = lockfile.parent().unwrap().join("Cargo.toml");
    let manifest = env::current_dir().unwrap().join(&manifest);
    let ws = try!(Workspace::new(&manifest, config));
    let specs = Packages::All.into_package_id_specs(&ws).chain_error(|| {
      human("failed to find specs? whats a spec?")
    })?;

    let override_name_and_ver_to_path: HashMap<(String, String), String> = overrides.into_iter()
      .map(|entry| ((entry.name, entry.version), entry.bazel_path))
      .collect();
    println!("overrides? {:?}", override_name_and_ver_to_path);


    let (_, resolve) = ops::resolve_ws_precisely(
            &ws,
            None,
            &[],
            false,
            false,
            &specs).chain_error(|| {
        human("failed to load pkg lockfile")
    })?;

    let hash = cargo::util::hex::short_hash(registry_id);
    let ident = registry_id.url().host().unwrap().to_string();
    let part = format!("{}-{}", ident, hash);

    let src = config.registry_source_path().join(&part);
    let ids = resolve.iter()
                     .filter(|id| id.source_id() == registry_id)
                     .cloned()
                     .collect::<Vec<_>>();
    let mut max = HashMap::new();
    for id in ids.iter() {
        let max = max.entry(id.name()).or_insert(id.version());
        *max = cmp::max(id.version(), *max)
    }


    let _ = fs::create_dir(&local_dst);

    let mut crate_decls = Vec::new();

    for id in ids.iter() {
        // First up, download the package
        let vers = format!("={}", id.version());
        let dep = try!(Dependency::parse_no_deprecated(id.name(),
                                                       Some(&vers[..]),
                                                       id.source_id()));
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
            return Err(human(format!("could not find package: {}", id)))
        }
        if vec.len() > 1 {
            return Err(human(format!("found too many packages: {}", id)))
        }
        /*
        try!(registry.download(id).chain_error(|| {
            human(format!("failed to download package from registry"))
        }));*/

        if override_name_and_ver_to_path.contains_key(&(id.name().to_owned(), id.version().to_string())) {
          continue
        }

        let dep_str = resolve.deps(id).into_iter()
          .map(|dep| {
            if override_name_and_ver_to_path.contains_key(&(dep.name().to_owned(), dep.version().to_string())) {
              format!("        \"{}\",\n", override_name_and_ver_to_path.get(&(dep.name().to_owned(), dep.version().to_string())).unwrap())
            } else {
              format!("        \"@io_cargo_{sanitized_name}//{sanitized_name}-{version}:{sanitized_name}\",\n", version=dep.version(), sanitized_name=dep.name().replace("-", "_"))
            }
          })
          .collect::<String>();

        // TODO(acmcarther): This will break as of cargo commit 50f1c172
        let feature_str = resolve.features(id)
          .cloned()
          .unwrap_or(HashSet::new())
          .iter()
          .map(|f| format!("        \"{}\",\n", f))
          .collect::<String>();

        let cargo_crate = format!(
r#"
new_crate_repository(
    name = "crates_io_{sanitized_crate_name}"
    crate_name = "{crate_name}",
    crate_version = "{crate_version}",
    build_file_content = """
package(default_visibility = ["//visibility:public"])

licenses(["notice"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)
rust_library(
    name = "{sanitized_crate_name}"
    srcs = glob(["lib.rs", "src/**/*.rs"]),
    deps = [
{comma_separated_cargo_deps}    ],
    rustc_flags = [
        "--cap-lints warn"
    ],
    crate_features = [
{comma_separated_features}    ],
)
"""
)
"#, crate_name=id.name(), sanitized_crate_name=id.name().replace("-", "_"), crate_version=id.version(), comma_separated_cargo_deps=dep_str, comma_separated_features=feature_str);
        crate_decls.push(cargo_crate)
    }

    let crate_decl_str = crate_decls.into_iter().collect::<String>();
    let workspace_str = format!(
r#"git_repository(
    name = "io_bazel_rules_rust",
    remote = "https://github.com/acmcarther/rules_rust.git",
    commit = "49a7345"
)
load(
    "@io_bazel_rules_rust//rust:rust.bzl",    "rust_repositories"
)
rust_repositories()
{}
"#, crate_decl_str);
    try!(try!(File::create(local_dst.join("WORKSPACE"))).write_all(workspace_str.as_bytes()));

    Ok(())
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

fn isnt_plus(chr: char) -> bool {
  chr != '+'
}
fn isnt_colon(chr: char) -> bool {
  chr != ':'
}
fn isnt_comma(chr: char) -> bool {
  chr != ','
}

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
