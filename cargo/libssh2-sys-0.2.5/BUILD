package(default_visibility = [ "//visibility:public" ])
licenses(["notice"])

load(
  "@io_bazel_rules_rust//rust:rust.bzl",
  "rust_library",
)

rust_library(
  name = "libssh2_sys",
  srcs = glob(["lib.rs", "src/**/*.rs"]),
  deps = [
    "//libz-sys-1.0.13:libz_sys",
    "//cmake-0.1.21:cmake",
    "//pkg-config-0.3.9:pkg_config",
    "//libc-0.2.21:libc",
    "//openssl-sys-0.9.7:openssl_sys",
  ],
  rustc_flags = [
    "--cap-lints warn"
  ],
  crate_features = [ 
  ],
)
