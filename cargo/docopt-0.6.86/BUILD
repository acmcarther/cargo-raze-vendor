package(default_visibility = [ "//visibility:public" ])
licenses(["notice"])

load(
  "@io_bazel_rules_rust//rust:rust.bzl",
  "rust_library",
)

rust_library(
  name = "docopt",
  srcs = glob(["lib.rs", "src/**/*.rs"]),
  deps = [
    "//regex-0.1.80:regex",
    "//lazy_static-0.2.4:lazy_static",
    "//strsim-0.5.2:strsim",
    "//rustc-serialize-0.3.22:rustc_serialize",
  ],
  rustc_flags = [
    "--cap-lints warn"
  ],
  crate_features = [ 
  ],
)
