package(default_visibility = [ "//visibility:public" ])
licenses(["notice"])

load(
  "@io_bazel_rules_rust//rust:rust.bzl",
  "rust_library",
)

rust_library(
  name = "env_logger",
  srcs = glob(["lib.rs", "src/**/*.rs"]),
  deps = [
    "//regex-0.1.80:regex",
    "//log-0.3.7:log",
  ],
  rustc_flags = [
    "--cap-lints warn"
  ],
  crate_features = [ 
    "default",
    "regex",
  ],
)
