# cargo-raze

A bazel rule generator for cargo dependencies.

This tool is experimental and no guarantees are made that it will work at all really. It works for the one example in this repo.

See the 'example' in [./cargo/test/BUILD](./cargo/test/BUILD)

## Example invocation

```
cargo run --raze ./cargo/ --overrides log-0.3.7:@custom-log//log:log
```

WORKSPACE file yielded into `./cargo/`

You will need to override any dependencies that use `build.rs` files. I suggest copying the generated rule and modifying the build_file_content as needed. Then, specify the override as prescribed above. It will be textually replaced in any instance of `@io_crates_*/:*`. Make sure the build file is accessible from the generated directory.
