package(default_visibility = [ "//visibility:public" ])
licenses(["notice"])

load(
  "@io_bazel_rules_rust//rust:rust.bzl",
  "rust_library",
)

rust_library(
  name = "thread_local",
  srcs = glob(["lib.rs", "src/**/*.rs"]),
  deps = [
    "//thread-id-2.0.0:thread_id",
  ],
  rustc_flags = [
    "--cap-lints warn"
  ],
  crate_features = [ 
  ],
)
