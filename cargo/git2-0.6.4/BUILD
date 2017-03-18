package(default_visibility = [ "//visibility:public" ])
licenses(["notice"])

load(
  "@io_bazel_rules_rust//rust:rust.bzl",
  "rust_library",
)

rust_library(
  name = "git2",
  srcs = glob(["lib.rs", "src/**/*.rs"]),
  deps = [
    "//openssl-probe-0.1.0:openssl_probe",
    "//libgit2-sys-0.6.7:libgit2_sys",
    "//libc-0.2.21:libc",
    "//openssl-sys-0.9.7:openssl_sys",
    "//url-1.4.0:url",
    "//bitflags-0.7.0:bitflags",
  ],
  rustc_flags = [
    "--cap-lints warn"
  ],
  crate_features = [ 
    "curl",
    "default",
    "ssh",
    "libgit2-sys",
    "openssl-probe",
    "openssl-sys",
    "https",
  ],
)
