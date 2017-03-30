extern crate cargo;
#[macro_use]
extern crate nom;
extern crate rustc_serialize;

use std::cmp;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::{self, File};
use std::str;
use std::io::Write;
use std::path::Path;

use cargo::ops::Packages;
use cargo::core::{SourceId, Dependency, Workspace};
use cargo::ops;
use cargo::CliResult;
use cargo::util::{human, CliError, ChainError, ToUrl, Config};

// Sidestep format!'s "string literals only" stipulation
macro_rules! NEW_CRATE_REPOSITORY_TEMPLATE {() => (r#"
new_crate_repository(
    name = "io_crates_{sanitized_crate_name}"
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
        "--cap-lints warn",
    ],
    crate_features = [
{comma_separated_features}    ],
)
"""
)
"#)}

macro_rules! NEW_HTTP_ARCHIVE_TEMPLATE {() => (r#"
new_http_archive(
    name = "io_crates_{sanitized_crate_name}",
    urls = [
      # Bazel's downloader renders HTTP page instead of downloading for some reason.
      #"https://crates.io/api/v1/crates/{crate_name}/{crate_version}/download"
      "https://crates-io.s3-us-west-1.amazonaws.com/crates/{crate_name}/{crate_name}-{crate_version}.crate",
    ],
    type = "tar.gz",
    strip_prefix = "{crate_name}-{crate_version}",
    build_file_content = """
package(default_visibility = ["//visibility:public"])

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
)
""",
)
alias(
    name = "{sanitized_crate_name}",
    actual = "@io_crates_{sanitized_crate_name}//:{sanitized_crate_name}",
)
"#)}


macro_rules! ROOT_WORKSPACE_TEMPLATE {() => (r#"#THIS IS A GENERATED FILE
# DO NOT MODIFY
# INSTEAD, RERUN cargo raze WITH YOUR PREFERRED RULE OVERRIDES
git_repository(
    name = "io_bazel_rules_rust",
    remote = "https://github.com/acmcarther/rules_rust.git",
    commit = "49a7345",
)

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_repositories",
)
rust_repositories()
{}"#)}

macro_rules! ROOT_HTTP_WORKSPACE_TEMPLATE {() => (r#"{}"#)}


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
Generate a Bazel WORKSPACE consisting of resolved dependencies for the current platform

Usage:
    cargo raze [options] [<path>]

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
    let local_dst = Path::new(options.arg_path.as_ref().unwrap_or(&default));

    // TODO(acmcarther): Fix unwrap. I'm unwrapping here temporarily because Nom's err is hard to
    // convert to CargoError
    let overrides = options.flag_overrides.as_ref().map(|f| parse_overrides(f).to_result().unwrap()).unwrap_or(Vec::new());
    try!(fs::create_dir_all(&local_dst).chain_error(|| {
        human(format!("failed to create: `{}`", local_dst.display()))
    }));
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

    let lockfile = Path::new(lockfile);
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

    let (_, resolve) = ops::resolve_ws_precisely(
            &ws,
            None,
            &[],
            false,
            false,
            &specs).chain_error(|| {
        human("failed to load pkg lockfile")
    })?;

    let mut ids = resolve.iter()
                     .filter(|id| *id.source_id() == registry_id)
                     .cloned()
                     .collect::<Vec<_>>();
    ids.sort_by_key(|id| id.name().to_owned());

    let mut max = HashMap::new();
    for id in ids.iter() {
        let max = max.entry(id.name()).or_insert(id.version());
        *max = cmp::max(id.version(), *max)
    }


    let _ = fs::create_dir(&local_dst);

    let mut crate_decls = Vec::new();

    for id in ids.iter() {
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
            return Err(CliError::new(human(format!("could not find package: {}", id)), 101))
        }
        if vec.len() > 1 {
            return Err(CliError::new(human(format!("found too many packages: {}", id)), 101))
        }

        // Skip generating new_crate_repository for overrides
        if override_name_and_ver_to_path.contains_key(&(id.name().to_owned(), id.version().to_string())) {
          continue
        }

        // TODO(acmcarther): Filter dev_dependencies and build_dependencies out of this list, and
        // emit a warning.
        let mut dep_strs = resolve.deps(id).into_iter()
          .map(|dep| {
            if override_name_and_ver_to_path.contains_key(&(dep.name().to_owned(), dep.version().to_string())) {
              format!("        \"{}\",\n", override_name_and_ver_to_path.get(&(dep.name().to_owned(), dep.version().to_string())).unwrap())
            } else {
              format!("        \"@io_crates_{sanitized_name}//:{sanitized_name}\",\n", sanitized_name=dep.name().replace("-", "_"))
            }
          })
          .collect::<Vec<String>>();

        dep_strs.sort();
        let dep_str = dep_strs.into_iter().collect::<String>();


        // TODO(acmcarther): This will break as of cargo commit 50f1c172
        let mut feature_strs = resolve.features(id)
          .cloned()
          .unwrap_or(HashSet::new())
          .iter()
          .map(|f| format!("        \"{}\",\n", f))
          .collect::<Vec<String>>();
        feature_strs.sort();
        let feature_str = feature_strs.into_iter().collect::<String>();

        let cargo_crate = format!(NEW_HTTP_ARCHIVE_TEMPLATE!(),
                                  crate_name=id.name(),
                                  sanitized_crate_name=id.name().replace("-", "_"),
                                  crate_version=id.version(),
                                  comma_separated_cargo_deps=dep_str,
                                  comma_separated_features=feature_str);
        crate_decls.push(cargo_crate)
    }

    let crate_decl_str = crate_decls.into_iter().collect::<String>();
    let workspace_str = format!(ROOT_WORKSPACE_TEMPLATE!(), crate_decl_str);
    let workspace_path = local_dst.join("WORKSPACE");
    try!(File::create(&workspace_path).and_then(|mut f| f.write_all(workspace_str.as_bytes())).chain_error(|| {
        human(format!("failed to create: `{}`", workspace_path.display()))
    }));

    Ok(None)
}
