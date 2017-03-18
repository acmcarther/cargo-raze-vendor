package(default_visibility = [ "//visibility:public" ])
licenses(["notice"])

load(
  "@io_bazel_rules_rust//rust:rust.bzl",
  "rust_library",
)

rust_library(
  name = "toml",
  srcs = glob(["lib.rs", "src/**/*.rs"]),
  deps = [
    "//rustc-serialize-0.3.22:rustc_serialize",
  ],
  rustc_flags = [
    "--cap-lints warn"
  ],
  crate_features = [ 
    "default",
    "rustc-serialize",
  ],
)
