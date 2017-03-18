package(default_visibility = [ "//visibility:public" ])
licenses(["notice"])

load(
  "@io_bazel_rules_rust//rust:rust.bzl",
  "rust_library",
)

rust_library(
  name = "git2_curl",
  srcs = glob(["lib.rs", "src/**/*.rs"]),
  deps = [
    "//curl-0.4.6:curl",
    "//log-0.3.7:log",
    "//url-1.4.0:url",
    "//git2-0.6.4:git2",
  ],
  rustc_flags = [
    "--cap-lints warn"
  ],
  crate_features = [ 
  ],
)
