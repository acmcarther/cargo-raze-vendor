package(default_visibility = [ "//visibility:public" ])
licenses(["notice"])

load(
  "@io_bazel_rules_rust//rust:rust.bzl",
  "rust_library",
)

rust_library(
  name = "tar",
  srcs = glob(["lib.rs", "src/**/*.rs"]),
  deps = [
    "//libc-0.2.21:libc",
    "//filetime-0.1.10:filetime",
  ],
  rustc_flags = [
    "--cap-lints warn"
  ],
  crate_features = [ 
  ],
)
