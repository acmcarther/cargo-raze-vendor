use cargo::CliResult;
use cargo::core::Source;
use cargo::core::Dependency;
use cargo::core::{Resolve, SourceId};
use cargo::util::Sha256;
use cargo::util::{human, ChainError, Config};
use cargo;
use common::RazePackage;
use common::DependencyWithPath;
use common;
use rustc_serialize::hex::ToHex;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Read;
use std::io::Write;
use std::io;
use std::path::Path;
use std::str;

macro_rules! VENDOR_DIR_WORKSPACE_SNIPPET {() => (r#"
local_repository(
    name = "{name}",
    path = __workspace_dir__ + "{path}"
)"#)}

pub fn materialize(
        resolve: &Resolve,
        registry: &mut Source,
        raze_packages: &Vec<RazePackage>,
        destination: &Path,
        registry_id: SourceId,
        config: &Config,
        override_name_and_ver_to_path: &HashMap<(String, String), String>,
        platform_triple: &str) -> CliResult{
    let _ = fs::create_dir(&destination);
    let hash = cargo::util::hex::short_hash(&registry_id);
    let ident = registry_id.url().host().unwrap().to_string();
    let part = format!("{}-{}", ident, hash);
    let src = config.registry_source_path().join(&part);

    for raze_package in raze_packages.iter() {
        let &RazePackage {ref id, ref features, ref package, ..} = raze_package;
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
        let dependencies_by_name = package.dependencies().iter().cloned()
          .map(|dep| (dep.name().to_owned(), dep))
          .collect::<HashMap<String, Dependency>>();

        let dependencies = resolve.deps(id).into_iter()
          .map(|dep| {
            assert!(dependencies_by_name.contains_key(dep.name()));
            let overriid = override_name_and_ver_to_path.get(&(dep.name().to_owned(), dep.version().to_string()));
            let path = match overriid {
              Some(override_path) => override_path.clone(),
              None => format!("//{crate_full_name}:{sanitized_name}",
                crate_full_name=format!("{}-{}", dep.name().replace("-", "_"), dep.version()),
                sanitized_name=dep.name().replace("-", "_"))
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
        let build_file_path = dst.join("BUILD");
        try!(File::create(&build_file_path).and_then(|mut f| f.write_all(build_file_content.as_bytes())).chain_error(|| {
            human(format!("failed to create: `{}`", build_file_path.display()))
        }));
    }
    let workspace_path = destination.join("WORKSPACE");
    try!(File::create(&workspace_path).chain_error(|| {
        human(format!("failed to create: `{}`", workspace_path.display()))
    }));

    println!("--vendor: Add the following snippet to your local workspace so bazel can find it: {}", format!(VENDOR_DIR_WORKSPACE_SNIPPET!(),
      name = "raze",
      path = destination.to_string_lossy()[1..].to_owned()));
    Ok(())
}

pub fn cp_r(src: &Path,
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

pub fn sha256(p: &Path) -> io::Result<String> {
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
