package(default_visibility = [ "//visibility:public" ])
licenses(["notice"])

load(
  "@io_bazel_rules_rust//rust:rust.bzl",
  "rust_library",
)

rust_library(
  name = "net2",
  srcs = glob(["lib.rs", "src/**/*.rs"]),
  deps = [
    "//kernel32-sys-0.2.2:kernel32_sys",
    "//libc-0.2.21:libc",
    "//ws2_32-sys-0.2.1:ws2_32_sys",
    "//cfg-if-0.1.0:cfg_if",
    "//winapi-0.2.8:winapi",
  ],
  rustc_flags = [
    "--cap-lints warn"
  ],
  crate_features = [ 
  ],
)
