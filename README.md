# cargo-raze

A bazel rule generator for cargo dependencies.

This tool is experimental and no guarantees are made that it will work at all really. It works for the one example in this repo. It was built as a spike to test out possible workflows for a proper tool implementation.

## Usage modes

### Generate WORKSPACE content

Example invocation (in a Cargo.toml+Cargo.lock bearing directory)
```
cargo install raze
cargo raze --overrides log-0.3.7:@custom-log//log:log --generate
```

A `generate` invocation will yield a series of new_http_archive() invocations into `./raze.WORKSPACE` that can be included in the repository's WORKSPACE file to automatically include the selected crates, with generated BUILD rules. An `overrides` flag is provided to allow users to override references of selected dependencies with their own defined rules. This is a useful feature when the automatically generated build rule fails to compile. Typically, rules that have complex system library dependencies, or leverage `build.rs` in interesting ways will require a custom override. Overrides in this manner will need to be accessible from your WORKSPACE, meaning they cannot live in the root workspace.

It is not suggested that users manually edit the generated rules, as those edits will have to be persisted when `generate` is rerun.

### Vendoring into ./raze/ or other directory

Example invocation (in a Cargo.toml+Cargo.lock bearing directory)
```
cargo install raze
cargo raze --overrides log-0.3.7:@custom-log//log:log --vendor ./raze
```

A `vendor` invocation will yield the full source code, supplemented with BUILD files into a selected directory. These rules function identically to normal source code. An `override` feature is provided to prevent vendoring of some dependency, in lieu of a user-customized dependency. This is typically necessary if the dependency leverages `build.rs` in some novel way.

## Supported features

- Build rule generation in vendored source format, or in WORKSPACE format
  - Vendored: [acmcarther/raze-manual-example](https://github.com/acmcarther/raze-manual-example)
  - WORKSPACE: [acmcarther/raze-autogen-example](https://github.com/acmcarther/raze-autogen-example)
- Platform specific dependencies for the environment `raze` is run on
- Feature flag propagation
- Dependency overriding

## Future features

- Platform detection (current platform is hardcoded to generic linux).
- Target platform specification, rather than current system platform
- `alias` rules for packages directly specified in Cargo.toml
- (potentially) Deferred BUILD file generation, to defer platform specific dependency resolution.
- (potentially) Generating `build.rs` shim rules, rather than forcing an `--override` when build_dependencies are present.
