package(default_visibility = [ "//visibility:public" ])
licenses(["notice"])

load(
  "@io_bazel_rules_rust//rust:rust.bzl",
  "rust_library",
)

rust_library(
  name = "openssl",
  srcs = glob(["lib.rs", "src/**/*.rs"]),
  deps = [
    "//foreign-types-0.2.0:foreign_types",
    "//libc-0.2.21:libc",
    "//openssl-sys-0.9.7:openssl_sys",
    "//lazy_static-0.2.4:lazy_static",
    "//bitflags-0.7.0:bitflags",
  ],
  rustc_flags = [
    "--cap-lints warn"
  ],
  crate_features = [ 
  ],
)
