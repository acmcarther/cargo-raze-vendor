package(default_visibility = [ "//visibility:public" ])
licenses(["notice"])

load(
  "@io_bazel_rules_rust//rust:rust.bzl",
  "rust_library",
)

rust_library(
  name = "miow",
  srcs = glob(["lib.rs", "src/**/*.rs"]),
  deps = [
    "//net2-0.2.27:net2",
    "//kernel32-sys-0.2.2:kernel32_sys",
    "//ws2_32-sys-0.2.1:ws2_32_sys",
    "//winapi-0.2.8:winapi",
  ],
  rustc_flags = [
    "--cap-lints warn"
  ],
  crate_features = [ 
  ],
)
