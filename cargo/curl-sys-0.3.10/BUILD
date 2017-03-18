package(default_visibility = [ "//visibility:public" ])
licenses(["notice"])

load(
  "@io_bazel_rules_rust//rust:rust.bzl",
  "rust_library",
)

rust_library(
  name = "curl_sys",
  srcs = glob(["lib.rs", "src/**/*.rs"]),
  deps = [
    "//libz-sys-1.0.13:libz_sys",
    "//pkg-config-0.3.9:pkg_config",
    "//gcc-0.3.43:gcc",
    "//libc-0.2.21:libc",
    "//openssl-sys-0.9.7:openssl_sys",
    "//winapi-0.2.8:winapi",
  ],
  rustc_flags = [
    "--cap-lints warn"
  ],
  crate_features = [ 
  ],
)
