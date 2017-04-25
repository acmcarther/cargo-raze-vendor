use cargo::CliResult;
use cargo::core::Resolve;
use cargo::core::Dependency;
use cargo::util::{human, ChainError};
use common::RazePackage;
use common::DependencyWithPath;
use common;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::str;

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

pub fn materialize(
        resolve: &Resolve,
        raze_packages: &Vec<RazePackage>,
        override_name_and_ver_to_path: &HashMap<(String, String), String>,
        platform_triple: &str) -> CliResult<Option<()>> {
    let mut bazel_workspace_entries = Vec::new();

    for raze_package in raze_packages.iter() {
        let &RazePackage {ref id, ref features, ref package, ..} = raze_package;
        let dependencies_by_name = package.dependencies().iter().cloned()
          .map(|dep| (dep.name().to_owned(), dep))
          .collect::<HashMap<String, Dependency>>();
        let dependencies = resolve.deps(id).into_iter()
          .map(|dep| {
            assert!(dependencies_by_name.contains_key(dep.name()));
            let overriid = override_name_and_ver_to_path.get(&(dep.name().to_owned(), dep.version().to_string()));
            let path = match overriid {
              Some(override_path) => override_path.clone(),
              None => format!("@io_crates_{sanitized_name}_{sanitized_crate_version}//:{sanitized_name}",
                sanitized_name=dep.name().replace("-", "_"),
                sanitized_crate_version=dep.version().to_string().replace(".", "_"))
            };
            DependencyWithPath {
              dependency: dependencies_by_name.get(dep.name())
                .expect("Dependencies from 'resolve' object should also be contained in 'package'")
                .clone(),
              path: path
            }
          })
          .collect::<Vec<_>>();
        let build_file_content = common::generate_build_file(
                                  id.name(),
                                  &dependencies,
                                  &features,
                                  platform_triple);

        let new_http_archive_decl = format!(NEW_HTTP_ARCHIVE_TEMPLATE!(),
                                  crate_name=id.name(),
                                  sanitized_crate_name=id.name().replace("-", "_"),
                                  crate_version=id.version(),
                                  sanitized_crate_version=id.version().to_string().replace(".", "_"),
                                  build_file_content=build_file_content);
        bazel_workspace_entries.push(new_http_archive_decl)
    }
    let bazel_workspace_content = bazel_workspace_entries.into_iter().collect::<String>();
    let workspace_path = Path::new("raze.WORKSPACE");
    try!(File::create(workspace_path).and_then(|mut f| f.write_all(bazel_workspace_content.as_bytes())).chain_error(|| {
        human(format!("failed to create: `{}`", workspace_path.display()))
    }));

    println!("--generate: A raze.WORKSPACE was created containing the generated dependencies. Integrate this into your existing WORKSPACE.");
    Ok(None)
}

