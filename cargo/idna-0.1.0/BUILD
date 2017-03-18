package(default_visibility = [ "//visibility:public" ])
licenses(["notice"])

load(
  "@io_bazel_rules_rust//rust:rust.bzl",
  "rust_library",
)

rust_library(
  name = "idna",
  srcs = glob(["lib.rs", "src/**/*.rs"]),
  deps = [
    "//unicode-normalization-0.1.4:unicode_normalization",
    "//matches-0.1.4:matches",
    "//unicode-bidi-0.2.5:unicode_bidi",
  ],
  rustc_flags = [
    "--cap-lints warn"
  ],
  crate_features = [ 
  ],
)
