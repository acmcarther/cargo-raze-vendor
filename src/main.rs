extern crate cargo;
#[macro_use]
extern crate nom;
extern crate rustc_serialize;

mod common;
mod vendor;
mod workspace;

use common::RazePackage;
use std::cmp;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::str;
use std::path::Path;

use cargo::CargoError;
use cargo::ops::Packages;
use cargo::core::dependency::Kind;
use cargo::core::SourceId;
use cargo::core::Workspace;
use cargo::ops;
use cargo::CliResult;
use cargo::util::{human, ChainError, ToUrl, Config};

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

// TODO(acmcarther): This will break soon. The method call is now "call_main_without_stdin"
const USAGE: &'static str = r#"
Generate a Bazel WORKSPACE consisting of resolved dependencies for the current platform

Usage:
    cargo proto-raze [options]

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
"#;

fn main() {

    let config = Config::default().unwrap();
    let args = env::args().collect::<Vec<_>>();
    let result = cargo::call_main_without_stdin(real_main, &config, USAGE, &args, false);

    match result {
        Err(e) => cargo::handle_cli_error(e, &mut *config.shell()),
        Ok(()) => {},
    }
}

fn real_main(options: Options, config: &Config) -> CliResult {
    try!(config.configure(options.flag_verbose,
                          options.flag_quiet,
                          &options.flag_color,
                          /* frozen = */ false,
                          /* locked = */ false));
    let vendor_destination = options.flag_vendor.as_ref().map(|p| Path::new(p));
    let generate_workspace_rules = options.flag_generate.unwrap_or(false);
    if vendor_destination.is_none() && !generate_workspace_rules {
      println!("Not generating vendored dependencies or workspace rules; exiting.");
      return Ok(());
    }

    if vendor_destination.is_some() && !vendor_destination.unwrap().starts_with("./") {
      println!("Please specify vendor destination as a relative path starting with './'");
      return Ok(());
    }

    let registry_id = try!(load_registry(options.flag_host, &config));
    try!(verify_lock_file_path(options.flag_sync));
    let lockfile = Path::new("Cargo.lock");
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

    let (packages, resolve) = ops::resolve_ws_precisely(
            &ws,
            None,
            &[],
            false,
            false,
            &specs).chain_error(|| {
        human("failed to load pkg lockfile")
    })?;


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

    let mut raze_packages = Vec::new();

    for id in package_ids.iter() {
        // Skip generating new_crate_repository for overrides
        if override_name_and_ver_to_path.contains_key(&(id.name().to_owned(), id.version().to_string())) {
          continue
        }

        raze_packages.push(RazePackage {
          id: id.clone(),
          package: packages.get(id).unwrap().clone(),
          // TODO(acmcarther): This will break as of cargo commit 50f1c172
          features: resolve.features(id)
            .cloned()
            .unwrap_or(HashSet::new()),
        });
    }
    {
        let mut printed_warning = false;
        for raze_pkg in raze_packages.iter() {
            if raze_pkg.package.dependencies().iter().any(|dep| dep.kind() == Kind::Build) {
                printed_warning = true;
                println!("WARNING: Crate <{}-{}> appears to contain a Build dependency.",
                    raze_pkg.id.name(), raze_pkg.id.version());

            }
        }
        if printed_warning {
            println!("WARNING: You will probably need to override Build dependent crates with the --override flag and provide a custom BUILD rule.");
        }
    }

    let platform_triple = config.rustc()?.host.clone();

    if generate_workspace_rules {
        try!(workspace::materialize(
            &resolve,
            &raze_packages,
            &override_name_and_ver_to_path,
            &platform_triple));
    }

    if vendor_destination.is_some() {
        try!(vendor::materialize(
            &resolve,
            &mut registry,
            &raze_packages,
            vendor_destination.unwrap(),
            registry_id,
            &config,
            &override_name_and_ver_to_path,
            &platform_triple));

    }

    Ok(())
}

fn load_registry(flag_host: Option<String>, config: &Config) -> Result<SourceId, Box<CargoError>> {
    let source_id_from_registry =
      flag_host.map(|s| s.to_url().map(|url| SourceId::for_registry(&url)).map_err(human));

    source_id_from_registry.unwrap_or_else(|| SourceId::crates_io(config))
}

fn verify_lock_file_path(flag_sync: Option<String>) -> CliResult {
    match flag_sync {
        Some(_) => Ok(()),
        None => {
            try!(fs::metadata("Cargo.lock").chain_error(|| {
              human("failed to find Cargo.lock. Please run `cargo generate_lockfile` first.")
            }));
            Ok(())
        }
    }
}

