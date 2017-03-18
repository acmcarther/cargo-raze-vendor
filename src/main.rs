extern crate cargo;
extern crate rustc_serialize;

use std::cmp;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::env;
use std::fs::{self, File};
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
}

const WORKSPACE_TEMPLATE: &'static str =
r#"git_repository(
  name = "io_bazel_rules_rust",
  remote = "https://github.com/acmcarther/rules_rust.git",
  commit = "49a7345"
)
load("@io_bazel_rules_rust//rust:rust.bzl", "rust_repositories")
rust_repositories()
"#;

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

    try!(sync(Path::new(lockfile), &path, &id, config).chain_error(|| {
        human("failed to sync")
    }));

    let mut workspace_file = File::create(Path::new(path).join("WORKSPACE")).chain_error(|| {
      human("failed to produce WORKSPACE file")
    })?;
    workspace_file.write_all(WORKSPACE_TEMPLATE.as_bytes()).chain_error(|| {
      human("failed to write to WORKSPACE file")
    })?;

    Ok(None)
}

fn sync(lockfile: &Path,
        local_dst: &Path,
        registry_id: &SourceId,
        config: &Config) -> CargoResult<()> {
    let mut registry = registry_id.load(config);
    let manifest = lockfile.parent().unwrap().join("Cargo.toml");
    let manifest = env::current_dir().unwrap().join(&manifest);
    let ws = try!(Workspace::new(&manifest, config));
    let specs = Packages::All.into_package_id_specs(&ws).chain_error(|| {
      human("failed to find specs? whats a spec?")
    })?;

    let (_, resolve) = ops::resolve_ws_precisely(
            &ws,
            None,
            &[],
            false,
            false,
            &specs).chain_error(|| {
        human("failed to load pkg lockfile")
    })?;

    println!("{:?}", registry_id);
    println!("{:?}", resolve);

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
        try!(registry.download(id).chain_error(|| {
            human(format!("failed to download package from registry"))
        }));

        // Next up, copy it to the vendor directory
        let name = format!("{}-{}", id.name(), id.version());
        let src = src.join(&name).into_path_unlocked();
        let dst_name = format!("{}-{}", id.name(), id.version());
        let dst = local_dst.join(&dst_name);
        let build_path = dst.join("BUILD");
        let _ = fs::remove_dir_all(&dst);
        let mut map = BTreeMap::new();
        try!(cp_r(&src, &dst, &dst, &mut map).chain_error(|| {
            human(format!("failed to copy over vendored sources for: {}", id))
        }));

        let dep_str = resolve.deps(id).into_iter()
          .map(|dep| format!("    \"//{name}-{version}:{sanitized_name}\",\n", name=dep.name(), version=dep.version(), sanitized_name=dep.name().replace("-", "_")))
          .collect::<String>();

        let alt_dep_str = resolve.deps(id).into_iter()
          .map(|dep| format!("    {{ name = \"{name}\", version = \"{version}\" }},\n", name=dep.name(), version=dep.version()))
          .collect::<String>();

        // TODO(acmcarther): This will break as of cargo commit 50f1c172
        let feature_str = resolve.features(id)
          .cloned()
          .unwrap_or(HashSet::new())
          .iter()
          .map(|f| format!("    \"{}\",\n", f))
          .collect::<String>();

        // Finally, emit the metadata about this package
        let build_file = format!(r#"package(default_visibility = [ "//visibility:public" ])
licenses(["notice"])

load(
  "@io_bazel_rules_rust//rust:rust.bzl",
  "rust_library",
)

rust_library(
  name = "{crate_name}",
  srcs = glob(["lib.rs", "src/**/*.rs"]),
  deps = [
{comma_separated_deps}  ],
  rustc_flags = [
    "--cap-lints warn"
  ],
  crate_features = [ 
{comma_separated_features}  ],
)
"#, crate_name=id.name().replace("-", "_"), comma_separated_deps=dep_str, comma_separated_features=feature_str);
        try!(try!(File::create(&build_path)).write_all(build_file.as_bytes()));

        let cargo_crate = println!(
r#"
cargo_crate(
  name = "{crate_name}"
  version = "{crate_version}"
  cargo_deps = [
{comma_separated_cargo_deps}  ],
  override_deps = [
  ],
  crate_features = [
{comma_separated_features}  ],
)
"#, crate_name=id.name(), crate_version=id.version(), comma_separated_cargo_deps=alt_dep_str, comma_separated_features=feature_str);
    }

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
