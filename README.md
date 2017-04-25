# cargo-raze

A bazel rule generator for cargo dependencies.

This tool is experimental and no guarantees are made that it will work at all really. It works for the one example in this repo. It was built as a spike to test out possible workflows for a proper tool implementation.

See the 'example' in [./cargo/test/BUILD](./cargo/test/BUILD)

The current implementation does not properly handle platform specific dependencies. The geneated rules in both the WORKSPACE or vendoring case will be specific to the platform that raze is run on.

## Usage modes

### Generate WORKSPACE content

Example invocation (in a Cargo.toml+Cargo.lock bearing directory)
```
cargo install raze
cargo raze --overrides log-0.3.7:@custom-log//log:log --generate
```

A `generate` invocation will yield a series of new_http_archive() invocations to stdout that can be included in the repository's WORKSPACE file to automatically include the selected crates, with generated BUILD rules. An `overrides` flag is provided to allow users to override references of selected dependencies with their own defined rules. This is a useful feature when the automatically generated build rule fails to compile. Typically, rules that have complex system library dependencies, or leverage `build.rs` in interesting ways will require a custom override.

It is not suggested that users manually edit the generated rules, as those edits will have to be persisted when `generate` is rerun.

### Vendoring into ./vendor/ or other directory

Example invocation (in a Cargo.toml+Cargo.lock bearing directory)
```
cargo install raze
cargo raze . --overrides log-0.3.7:@custom-log//log:log --vendor
```

A `vendor` invocation will yield the full source code, supplemented with BUILD files into a selected directory, or `vendor` by default. These rules function identically to normal source code. An `override` feature is provided to prevent vendoring of some dependency, in lieu of a user-customized dependency. This is typically necessary if the dependency leverages `build.rs` in some novel way.
