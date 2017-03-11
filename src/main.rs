extern crate cargo;
extern crate rustc_serialize;
use std::cmp;
use std::collections::{BTreeMap, HashMap};
use std::env;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;

use rustc_serialize::hex::ToHex;
use rustc_serialize::json::{self, ToJson};

use cargo::core::{SourceId, Dependency, Workspace};
use cargo::CliResult;
use cargo::util::{human, ChainError, ToUrl, Config, CargoResult};
use cargo::util::Sha256;

#[derive(RustcDecodable)]
struct Options {
  arg_path: Option<String>,
  flag_verbose: u32,
  flag_host: Option<String>,
  flag_sync: Option<String>,
  flag_color: Option<String>,
  flag_quiet: Option<bool>,
}

pub fn main() {
  cargo::execute_main_without_stdin(real_main, false, r#"
Vendor all dependencies for a project with BUILD rules.

Usage:
    cargo raze vendor [options] [<path>]

Options:
    -h, --help       Print this message
    -s, --sync LOCK  Sync the registry with LOCK
    --host HOST      Registry index to sync with
    -v, --verbose    Use verbose output
    -q, --quiet      No output printed to stdout
    --color WHEN     Coloring: auto, always, never
"#)
}

fn real_main(options: Options, config: &Config) -> CliResult<Option<()>> {
  // Generate Cargo.lock
  // Download all sources into ./sources
  // Resolve dependency graph (for explicit or implicit target)
  // Emit build rules, using actual build commands
  // Emit direct dependencies
  try!(config.configure(options.flag_verbose,
                        options.flag_quiet,
                        &options.flag_color,
                        /* frozen = */ false,
                        /* locked = */ false));


  let registry_id = try!(options.flag_host.map(|s| {
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

  let mut registry = registry_id.load(config);
  let lockfile_path = Path::new(lockfile);
  let manifest = env::current_dir().unwrap().join(lockfile_path.parent().unwrap().join("Cargo.toml"));
  let ws = try!(Workspace::new(&manifest, config));
  let resolve = try!(cargo::ops::load_pkg_lockfile(&ws).chain_error(|| {
    human("failed to load pkg lockfile")
  }));
  let resolve = try!(resolve.chain_error(|| {
    human(format!("lock file `{}` does not exist", lockfile_path.display()))
  }));

  let hash = cargo::util::hex::short_hash(&registry_id);
  let ids = resolve.iter()
                   .filter(|id| id.source_id() == &registry_id)
                   .cloned()
                   .collect::<Vec<_>>();

  println!("{:?}", resolve);
  println!("{}", hash);
  println!("{:?}", ids);

  for id in ids.iter() {
    // First up, download the package
    let vers = format!("={}", id.version());
    let dep = try!(Dependency::parse_no_deprecated(id.name(),
      Some(&vers[..]),
      id.source_id()));
    let mut vec = try!(registry.query(&dep));
    println!("{:?}", vers);
    println!("{:?}", dep);
    println!("{:?}", vec);
    // TODO(acmcarther): use error chain
    assert!(vec.len() == 1);
    vec.retain(|version| {
      version.package_id().version().to_string() == id.version().to_string()
    });
    /*
    if vec.len() == 0 {
      return Err(human(format!("could not find package: {}", id)))
    }
    if vec.len() > 1 {
      return Err(human(format!("found too many packages: {}", id)))
    }
    */
    try!(registry.download(id).chain_error(|| {
      human(format!("failed to download package from registry"))
    }));
  }

  Ok(None)
}
