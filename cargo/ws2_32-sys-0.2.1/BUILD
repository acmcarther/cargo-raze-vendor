package(default_visibility = [ "//visibility:public" ])
licenses(["notice"])

load(
  "@io_bazel_rules_rust//rust:rust.bzl",
  "rust_library",
)

rust_library(
  name = "ws2_32_sys",
  srcs = glob(["lib.rs", "src/**/*.rs"]),
  deps = [
    "//winapi-build-0.1.1:winapi_build",
    "//winapi-0.2.8:winapi",
  ],
  rustc_flags = [
    "--cap-lints warn"
  ],
  crate_features = [ 
  ],
)
