package(default_visibility = ["//visibility:public"])


load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_binary",
)

rust_binary(
    name = "test",
    srcs = glob(["src/**/.*.rs"]),
    crate_root = "src/main.rs",
    deps = [
        "@io_crates_tar//:tar"
    ]
)
