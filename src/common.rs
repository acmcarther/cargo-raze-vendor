use std::collections::HashSet;
use std::str;
use std::str::FromStr;
use cargo::core::Package;
use cargo::util::Cfg;
use cargo::core::Dependency;
use cargo::core::dependency::Kind;
use cargo::core::PackageId;

#[derive(Debug)]
pub struct RazePackage {
  pub id: PackageId,
  pub package: Package,
  pub features: HashSet<String>,
}

#[derive(Debug)]
pub struct DependencyWithPath {
  pub dependency: Dependency,
  pub path: String
}


fn generic_linux_cfgs() -> Vec<Cfg> {
    // TODO(acmcarther): use output of rustc, similar to
    // cargo::ops::cargo_rustc::context::Context::probe_target_info_kind
    // https://github.com/rust-lang/cargo/blob/f5348cc321a032db95cd18e3129a4392d2e0a892/src/cargo/ops/cargo_rustc/context.rs#L199
    let hardcoded_properties =
r#"debug_assertions
target_arch="x86_64"
target_endian="little"
target_env="gnu"
target_family="unix"
target_feature="sse"
target_feature="sse2"
target_has_atomic="16"
target_has_atomic="32"
target_has_atomic="64"
target_has_atomic="8"
target_has_atomic="ptr"
target_os="linux"
target_pointer_width="64"
target_thread_local
target_vendor="unknown"
unix"#;
    hardcoded_properties.lines()
      .map(Cfg::from_str)
      .map(|cfg| cfg.expect("hardcoded values should be parsable"))
      .collect()
}


pub fn generate_build_file(crate_name: &str, dependencies: &Vec<DependencyWithPath>, feature_strings: &HashSet<String>, platform_triple: &str) -> String {
    let mut unused_dependencies_with_reason: Vec<String> = Vec::new();
    let mut dependency_strs = Vec::new();

    for dep in dependencies.iter() {
        let &DependencyWithPath { ref dependency, ref path } = dep;
        if dependency.kind() != Kind::Normal {
            unused_dependencies_with_reason.push(
              format!("Dependency <{}> is <{:?}> dependency", path.clone(), dependency.kind()));
            continue
        }

        if dependency.platform().map(|p| !p.matches(platform_triple, Some(&generic_linux_cfgs()))).unwrap_or(false) {
            unused_dependencies_with_reason.push(
              format!("Dependency <{}> is for alternate platform: <{}>", path.clone(), dependency.platform().unwrap()));
            continue
        }

        dependency_strs.push(path);
    }

    // Make output "stable" by sorting
    dependency_strs.sort();
    unused_dependencies_with_reason.sort();

    let unused_dependencies_str = unused_dependencies_with_reason.into_iter().map(|msg| format!("- {}\n", msg)).collect::<String>();
    format!(r#"'''
WARNING: THIS IS GENERATED CODE!
DO NOT MODIFY!
Instead, rerun raze with different arguments.

{notes}
'''
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
{comma_separated_deps}    ],
    rustc_flags = [
        "--cap-lints warn",
    ],
    crate_features = [
{comma_separated_features}    ],
)"#,
    sanitized_crate_name = crate_name.replace("-", "_"),
    comma_separated_deps = dependency_strs.into_iter()
        .map(|dep_str| format!("        \"{}\",\n", dep_str))
        .collect::<String>(),
    comma_separated_features = bazelize_features(feature_strings),
    notes = format!(
"Unused dependencies from cargo: [
{unused_dependencies}]"
, unused_dependencies=unused_dependencies_str))
}

fn bazelize_features(features: &HashSet<String>) -> String {
    let mut feature_strs = features
      .iter()
      .map(|f| format!("        \"{}\",\n", f))
      .collect::<Vec<String>>();
    feature_strs.sort();
    return feature_strs.into_iter().collect::<String>();
}
