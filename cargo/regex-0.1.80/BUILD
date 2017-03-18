package(default_visibility = [ "//visibility:public" ])
licenses(["notice"])

load(
  "@io_bazel_rules_rust//rust:rust.bzl",
  "rust_library",
)

rust_library(
  name = "regex",
  srcs = glob(["lib.rs", "src/**/*.rs"]),
  deps = [
    "//aho-corasick-0.5.3:aho_corasick",
    "//utf8-ranges-0.1.3:utf8_ranges",
    "//regex-syntax-0.3.9:regex_syntax",
    "//memchr-0.1.11:memchr",
    "//thread_local-0.2.7:thread_local",
  ],
  rustc_flags = [
    "--cap-lints warn"
  ],
  crate_features = [ 
  ],
)
