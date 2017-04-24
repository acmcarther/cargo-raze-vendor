git_repository(
  name = "io_bazel_rules_rust",
  remote = "https://github.com/acmcarther/rules_rust.git",
  commit = "49a7345"
)
load("@io_bazel_rules_rust//rust:rust.bzl", "rust_repositories")
rust_repositories()

local_repository(
    name = "vendor",
    path = __workspace_dir__ + "/vendor"
)

new_http_archive(
    name = "io_crates_advapi32_sys_0_2_0",
    urls = [
      # Bazel's downloader renders HTTP page instead of downloading for some reason.
      #"https://crates.io/api/v1/crates/advapi32-sys/0.2.0/download"
      "https://crates-io.s3-us-west-1.amazonaws.com/crates/advapi32-sys/advapi32-sys-0.2.0.crate",
    ],
    type = "tar.gz",
    strip_prefix = "advapi32-sys-0.2.0",
    build_file_content = """
package(default_visibility = ["//visibility:public"])

licenses(["notice"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)
rust_library(
    name = "advapi32_sys",
    srcs = glob(["lib.rs", "src/**/*.rs"]),
    deps = [
        "@io_crates_winapi_0_2_8//:winapi",
        "@io_crates_winapi_build_0_1_1//:winapi_build",
    ],
    rustc_flags = [
        "--cap-lints warn",
    ],
    crate_features = [
    ],
)
""",
)

new_http_archive(
    name = "io_crates_aho_corasick_0_5_3",
    urls = [
      # Bazel's downloader renders HTTP page instead of downloading for some reason.
      #"https://crates.io/api/v1/crates/aho-corasick/0.5.3/download"
      "https://crates-io.s3-us-west-1.amazonaws.com/crates/aho-corasick/aho-corasick-0.5.3.crate",
    ],
    type = "tar.gz",
    strip_prefix = "aho-corasick-0.5.3",
    build_file_content = """
package(default_visibility = ["//visibility:public"])

licenses(["notice"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)
rust_library(
    name = "aho_corasick",
    srcs = glob(["lib.rs", "src/**/*.rs"]),
    deps = [
        "@io_crates_memchr_0_1_11//:memchr",
    ],
    rustc_flags = [
        "--cap-lints warn",
    ],
    crate_features = [
    ],
)
""",
)

new_http_archive(
    name = "io_crates_bitflags_0_7_0",
    urls = [
      # Bazel's downloader renders HTTP page instead of downloading for some reason.
      #"https://crates.io/api/v1/crates/bitflags/0.7.0/download"
      "https://crates-io.s3-us-west-1.amazonaws.com/crates/bitflags/bitflags-0.7.0.crate",
    ],
    type = "tar.gz",
    strip_prefix = "bitflags-0.7.0",
    build_file_content = """
package(default_visibility = ["//visibility:public"])

licenses(["notice"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)
rust_library(
    name = "bitflags",
    srcs = glob(["lib.rs", "src/**/*.rs"]),
    deps = [
    ],
    rustc_flags = [
        "--cap-lints warn",
    ],
    crate_features = [
    ],
)
""",
)

new_http_archive(
    name = "io_crates_bitflags_0_8_2",
    urls = [
      # Bazel's downloader renders HTTP page instead of downloading for some reason.
      #"https://crates.io/api/v1/crates/bitflags/0.8.2/download"
      "https://crates-io.s3-us-west-1.amazonaws.com/crates/bitflags/bitflags-0.8.2.crate",
    ],
    type = "tar.gz",
    strip_prefix = "bitflags-0.8.2",
    build_file_content = """
package(default_visibility = ["//visibility:public"])

licenses(["notice"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)
rust_library(
    name = "bitflags",
    srcs = glob(["lib.rs", "src/**/*.rs"]),
    deps = [
    ],
    rustc_flags = [
        "--cap-lints warn",
    ],
    crate_features = [
    ],
)
""",
)

new_http_archive(
    name = "io_crates_cfg_if_0_1_0",
    urls = [
      # Bazel's downloader renders HTTP page instead of downloading for some reason.
      #"https://crates.io/api/v1/crates/cfg-if/0.1.0/download"
      "https://crates-io.s3-us-west-1.amazonaws.com/crates/cfg-if/cfg-if-0.1.0.crate",
    ],
    type = "tar.gz",
    strip_prefix = "cfg-if-0.1.0",
    build_file_content = """
package(default_visibility = ["//visibility:public"])

licenses(["notice"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)
rust_library(
    name = "cfg_if",
    srcs = glob(["lib.rs", "src/**/*.rs"]),
    deps = [
    ],
    rustc_flags = [
        "--cap-lints warn",
    ],
    crate_features = [
    ],
)
""",
)

new_http_archive(
    name = "io_crates_cmake_0_1_22",
    urls = [
      # Bazel's downloader renders HTTP page instead of downloading for some reason.
      #"https://crates.io/api/v1/crates/cmake/0.1.22/download"
      "https://crates-io.s3-us-west-1.amazonaws.com/crates/cmake/cmake-0.1.22.crate",
    ],
    type = "tar.gz",
    strip_prefix = "cmake-0.1.22",
    build_file_content = """
package(default_visibility = ["//visibility:public"])

licenses(["notice"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)
rust_library(
    name = "cmake",
    srcs = glob(["lib.rs", "src/**/*.rs"]),
    deps = [
        "@io_crates_gcc_0_3_45//:gcc",
    ],
    rustc_flags = [
        "--cap-lints warn",
    ],
    crate_features = [
    ],
)
""",
)

new_http_archive(
    name = "io_crates_crossbeam_0_2_10",
    urls = [
      # Bazel's downloader renders HTTP page instead of downloading for some reason.
      #"https://crates.io/api/v1/crates/crossbeam/0.2.10/download"
      "https://crates-io.s3-us-west-1.amazonaws.com/crates/crossbeam/crossbeam-0.2.10.crate",
    ],
    type = "tar.gz",
    strip_prefix = "crossbeam-0.2.10",
    build_file_content = """
package(default_visibility = ["//visibility:public"])

licenses(["notice"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)
rust_library(
    name = "crossbeam",
    srcs = glob(["lib.rs", "src/**/*.rs"]),
    deps = [
    ],
    rustc_flags = [
        "--cap-lints warn",
    ],
    crate_features = [
    ],
)
""",
)

new_http_archive(
    name = "io_crates_curl_0_4_6",
    urls = [
      # Bazel's downloader renders HTTP page instead of downloading for some reason.
      #"https://crates.io/api/v1/crates/curl/0.4.6/download"
      "https://crates-io.s3-us-west-1.amazonaws.com/crates/curl/curl-0.4.6.crate",
    ],
    type = "tar.gz",
    strip_prefix = "curl-0.4.6",
    build_file_content = """
package(default_visibility = ["//visibility:public"])

licenses(["notice"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)
rust_library(
    name = "curl",
    srcs = glob(["lib.rs", "src/**/*.rs"]),
    deps = [
        "@io_crates_curl_sys_0_3_10//:curl_sys",
        "@io_crates_libc_0_2_21//:libc",
        "@io_crates_openssl_probe_0_1_1//:openssl_probe",
        "@io_crates_openssl_sys_0_9_11//:openssl_sys",
        "@io_crates_winapi_0_2_8//:winapi",
    ],
    rustc_flags = [
        "--cap-lints warn",
    ],
    crate_features = [
    ],
)
""",
)

new_http_archive(
    name = "io_crates_curl_sys_0_3_10",
    urls = [
      # Bazel's downloader renders HTTP page instead of downloading for some reason.
      #"https://crates.io/api/v1/crates/curl-sys/0.3.10/download"
      "https://crates-io.s3-us-west-1.amazonaws.com/crates/curl-sys/curl-sys-0.3.10.crate",
    ],
    type = "tar.gz",
    strip_prefix = "curl-sys-0.3.10",
    build_file_content = """
package(default_visibility = ["//visibility:public"])

licenses(["notice"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)
rust_library(
    name = "curl_sys",
    srcs = glob(["lib.rs", "src/**/*.rs"]),
    deps = [
        "@io_crates_gcc_0_3_45//:gcc",
        "@io_crates_libc_0_2_21//:libc",
        "@io_crates_libz_sys_1_0_13//:libz_sys",
        "@io_crates_openssl_sys_0_9_11//:openssl_sys",
        "@io_crates_pkg_config_0_3_9//:pkg_config",
        "@io_crates_winapi_0_2_8//:winapi",
    ],
    rustc_flags = [
        "--cap-lints warn",
    ],
    crate_features = [
    ],
)
""",
)

new_http_archive(
    name = "io_crates_docopt_0_6_86",
    urls = [
      # Bazel's downloader renders HTTP page instead of downloading for some reason.
      #"https://crates.io/api/v1/crates/docopt/0.6.86/download"
      "https://crates-io.s3-us-west-1.amazonaws.com/crates/docopt/docopt-0.6.86.crate",
    ],
    type = "tar.gz",
    strip_prefix = "docopt-0.6.86",
    build_file_content = """
package(default_visibility = ["//visibility:public"])

licenses(["notice"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)
rust_library(
    name = "docopt",
    srcs = glob(["lib.rs", "src/**/*.rs"]),
    deps = [
        "@io_crates_lazy_static_0_2_8//:lazy_static",
        "@io_crates_regex_0_1_80//:regex",
        "@io_crates_rustc_serialize_0_3_24//:rustc_serialize",
        "@io_crates_strsim_0_5_2//:strsim",
    ],
    rustc_flags = [
        "--cap-lints warn",
    ],
    crate_features = [
    ],
)
""",
)

new_http_archive(
    name = "io_crates_env_logger_0_3_5",
    urls = [
      # Bazel's downloader renders HTTP page instead of downloading for some reason.
      #"https://crates.io/api/v1/crates/env_logger/0.3.5/download"
      "https://crates-io.s3-us-west-1.amazonaws.com/crates/env_logger/env_logger-0.3.5.crate",
    ],
    type = "tar.gz",
    strip_prefix = "env_logger-0.3.5",
    build_file_content = """
package(default_visibility = ["//visibility:public"])

licenses(["notice"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)
rust_library(
    name = "env_logger",
    srcs = glob(["lib.rs", "src/**/*.rs"]),
    deps = [
        "@io_crates_log_0_3_7//:log",
        "@io_crates_regex_0_1_80//:regex",
    ],
    rustc_flags = [
        "--cap-lints warn",
    ],
    crate_features = [
        "default",
        "regex",
    ],
)
""",
)

new_http_archive(
    name = "io_crates_filetime_0_1_10",
    urls = [
      # Bazel's downloader renders HTTP page instead of downloading for some reason.
      #"https://crates.io/api/v1/crates/filetime/0.1.10/download"
      "https://crates-io.s3-us-west-1.amazonaws.com/crates/filetime/filetime-0.1.10.crate",
    ],
    type = "tar.gz",
    strip_prefix = "filetime-0.1.10",
    build_file_content = """
package(default_visibility = ["//visibility:public"])

licenses(["notice"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)
rust_library(
    name = "filetime",
    srcs = glob(["lib.rs", "src/**/*.rs"]),
    deps = [
        "@io_crates_libc_0_2_21//:libc",
    ],
    rustc_flags = [
        "--cap-lints warn",
    ],
    crate_features = [
    ],
)
""",
)

new_http_archive(
    name = "io_crates_flate2_0_2_19",
    urls = [
      # Bazel's downloader renders HTTP page instead of downloading for some reason.
      #"https://crates.io/api/v1/crates/flate2/0.2.19/download"
      "https://crates-io.s3-us-west-1.amazonaws.com/crates/flate2/flate2-0.2.19.crate",
    ],
    type = "tar.gz",
    strip_prefix = "flate2-0.2.19",
    build_file_content = """
package(default_visibility = ["//visibility:public"])

licenses(["notice"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)
rust_library(
    name = "flate2",
    srcs = glob(["lib.rs", "src/**/*.rs"]),
    deps = [
        "@io_crates_libc_0_2_21//:libc",
        "@io_crates_miniz_sys_0_1_9//:miniz_sys",
    ],
    rustc_flags = [
        "--cap-lints warn",
    ],
    crate_features = [
        "default",
        "miniz-sys",
    ],
)
""",
)

new_http_archive(
    name = "io_crates_foreign_types_0_2_0",
    urls = [
      # Bazel's downloader renders HTTP page instead of downloading for some reason.
      #"https://crates.io/api/v1/crates/foreign-types/0.2.0/download"
      "https://crates-io.s3-us-west-1.amazonaws.com/crates/foreign-types/foreign-types-0.2.0.crate",
    ],
    type = "tar.gz",
    strip_prefix = "foreign-types-0.2.0",
    build_file_content = """
package(default_visibility = ["//visibility:public"])

licenses(["notice"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)
rust_library(
    name = "foreign_types",
    srcs = glob(["lib.rs", "src/**/*.rs"]),
    deps = [
    ],
    rustc_flags = [
        "--cap-lints warn",
    ],
    crate_features = [
    ],
)
""",
)

new_http_archive(
    name = "io_crates_fs2_0_3_0",
    urls = [
      # Bazel's downloader renders HTTP page instead of downloading for some reason.
      #"https://crates.io/api/v1/crates/fs2/0.3.0/download"
      "https://crates-io.s3-us-west-1.amazonaws.com/crates/fs2/fs2-0.3.0.crate",
    ],
    type = "tar.gz",
    strip_prefix = "fs2-0.3.0",
    build_file_content = """
package(default_visibility = ["//visibility:public"])

licenses(["notice"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)
rust_library(
    name = "fs2",
    srcs = glob(["lib.rs", "src/**/*.rs"]),
    deps = [
        "@io_crates_kernel32_sys_0_2_2//:kernel32_sys",
        "@io_crates_libc_0_2_21//:libc",
        "@io_crates_winapi_0_2_8//:winapi",
    ],
    rustc_flags = [
        "--cap-lints warn",
    ],
    crate_features = [
    ],
)
""",
)

new_http_archive(
    name = "io_crates_gcc_0_3_45",
    urls = [
      # Bazel's downloader renders HTTP page instead of downloading for some reason.
      #"https://crates.io/api/v1/crates/gcc/0.3.45/download"
      "https://crates-io.s3-us-west-1.amazonaws.com/crates/gcc/gcc-0.3.45.crate",
    ],
    type = "tar.gz",
    strip_prefix = "gcc-0.3.45",
    build_file_content = """
package(default_visibility = ["//visibility:public"])

licenses(["notice"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)
rust_library(
    name = "gcc",
    srcs = glob(["lib.rs", "src/**/*.rs"]),
    deps = [
    ],
    rustc_flags = [
        "--cap-lints warn",
    ],
    crate_features = [
    ],
)
""",
)

new_http_archive(
    name = "io_crates_gdi32_sys_0_2_0",
    urls = [
      # Bazel's downloader renders HTTP page instead of downloading for some reason.
      #"https://crates.io/api/v1/crates/gdi32-sys/0.2.0/download"
      "https://crates-io.s3-us-west-1.amazonaws.com/crates/gdi32-sys/gdi32-sys-0.2.0.crate",
    ],
    type = "tar.gz",
    strip_prefix = "gdi32-sys-0.2.0",
    build_file_content = """
package(default_visibility = ["//visibility:public"])

licenses(["notice"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)
rust_library(
    name = "gdi32_sys",
    srcs = glob(["lib.rs", "src/**/*.rs"]),
    deps = [
        "@io_crates_winapi_0_2_8//:winapi",
        "@io_crates_winapi_build_0_1_1//:winapi_build",
    ],
    rustc_flags = [
        "--cap-lints warn",
    ],
    crate_features = [
    ],
)
""",
)

new_http_archive(
    name = "io_crates_git2_0_6_4",
    urls = [
      # Bazel's downloader renders HTTP page instead of downloading for some reason.
      #"https://crates.io/api/v1/crates/git2/0.6.4/download"
      "https://crates-io.s3-us-west-1.amazonaws.com/crates/git2/git2-0.6.4.crate",
    ],
    type = "tar.gz",
    strip_prefix = "git2-0.6.4",
    build_file_content = """
package(default_visibility = ["//visibility:public"])

licenses(["notice"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)
rust_library(
    name = "git2",
    srcs = glob(["lib.rs", "src/**/*.rs"]),
    deps = [
        "@io_crates_bitflags_0_7_0//:bitflags",
        "@io_crates_libc_0_2_21//:libc",
        "@io_crates_libgit2_sys_0_6_7//:libgit2_sys",
        "@io_crates_openssl_probe_0_1_1//:openssl_probe",
        "@io_crates_openssl_sys_0_9_11//:openssl_sys",
        "@io_crates_url_1_4_0//:url",
    ],
    rustc_flags = [
        "--cap-lints warn",
    ],
    crate_features = [
        "curl",
        "default",
        "https",
        "libgit2-sys",
        "openssl-probe",
        "openssl-sys",
        "ssh",
    ],
)
""",
)

new_http_archive(
    name = "io_crates_git2_curl_0_7_0",
    urls = [
      # Bazel's downloader renders HTTP page instead of downloading for some reason.
      #"https://crates.io/api/v1/crates/git2-curl/0.7.0/download"
      "https://crates-io.s3-us-west-1.amazonaws.com/crates/git2-curl/git2-curl-0.7.0.crate",
    ],
    type = "tar.gz",
    strip_prefix = "git2-curl-0.7.0",
    build_file_content = """
package(default_visibility = ["//visibility:public"])

licenses(["notice"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)
rust_library(
    name = "git2_curl",
    srcs = glob(["lib.rs", "src/**/*.rs"]),
    deps = [
        "@io_crates_curl_0_4_6//:curl",
        "@io_crates_git2_0_6_4//:git2",
        "@io_crates_log_0_3_7//:log",
        "@io_crates_url_1_4_0//:url",
    ],
    rustc_flags = [
        "--cap-lints warn",
    ],
    crate_features = [
    ],
)
""",
)

new_http_archive(
    name = "io_crates_glob_0_2_11",
    urls = [
      # Bazel's downloader renders HTTP page instead of downloading for some reason.
      #"https://crates.io/api/v1/crates/glob/0.2.11/download"
      "https://crates-io.s3-us-west-1.amazonaws.com/crates/glob/glob-0.2.11.crate",
    ],
    type = "tar.gz",
    strip_prefix = "glob-0.2.11",
    build_file_content = """
package(default_visibility = ["//visibility:public"])

licenses(["notice"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)
rust_library(
    name = "glob",
    srcs = glob(["lib.rs", "src/**/*.rs"]),
    deps = [
    ],
    rustc_flags = [
        "--cap-lints warn",
    ],
    crate_features = [
    ],
)
""",
)

new_http_archive(
    name = "io_crates_idna_0_1_1",
    urls = [
      # Bazel's downloader renders HTTP page instead of downloading for some reason.
      #"https://crates.io/api/v1/crates/idna/0.1.1/download"
      "https://crates-io.s3-us-west-1.amazonaws.com/crates/idna/idna-0.1.1.crate",
    ],
    type = "tar.gz",
    strip_prefix = "idna-0.1.1",
    build_file_content = """
package(default_visibility = ["//visibility:public"])

licenses(["notice"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)
rust_library(
    name = "idna",
    srcs = glob(["lib.rs", "src/**/*.rs"]),
    deps = [
        "@io_crates_matches_0_1_4//:matches",
        "@io_crates_unicode_bidi_0_2_5//:unicode_bidi",
        "@io_crates_unicode_normalization_0_1_4//:unicode_normalization",
    ],
    rustc_flags = [
        "--cap-lints warn",
    ],
    crate_features = [
    ],
)
""",
)

new_http_archive(
    name = "io_crates_kernel32_sys_0_2_2",
    urls = [
      # Bazel's downloader renders HTTP page instead of downloading for some reason.
      #"https://crates.io/api/v1/crates/kernel32-sys/0.2.2/download"
      "https://crates-io.s3-us-west-1.amazonaws.com/crates/kernel32-sys/kernel32-sys-0.2.2.crate",
    ],
    type = "tar.gz",
    strip_prefix = "kernel32-sys-0.2.2",
    build_file_content = """
package(default_visibility = ["//visibility:public"])

licenses(["notice"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)
rust_library(
    name = "kernel32_sys",
    srcs = glob(["lib.rs", "src/**/*.rs"]),
    deps = [
        "@io_crates_winapi_0_2_8//:winapi",
        "@io_crates_winapi_build_0_1_1//:winapi_build",
    ],
    rustc_flags = [
        "--cap-lints warn",
    ],
    crate_features = [
    ],
)
""",
)

new_http_archive(
    name = "io_crates_lazy_static_0_2_8",
    urls = [
      # Bazel's downloader renders HTTP page instead of downloading for some reason.
      #"https://crates.io/api/v1/crates/lazy_static/0.2.8/download"
      "https://crates-io.s3-us-west-1.amazonaws.com/crates/lazy_static/lazy_static-0.2.8.crate",
    ],
    type = "tar.gz",
    strip_prefix = "lazy_static-0.2.8",
    build_file_content = """
package(default_visibility = ["//visibility:public"])

licenses(["notice"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)
rust_library(
    name = "lazy_static",
    srcs = glob(["lib.rs", "src/**/*.rs"]),
    deps = [
    ],
    rustc_flags = [
        "--cap-lints warn",
    ],
    crate_features = [
    ],
)
""",
)

new_http_archive(
    name = "io_crates_libc_0_2_21",
    urls = [
      # Bazel's downloader renders HTTP page instead of downloading for some reason.
      #"https://crates.io/api/v1/crates/libc/0.2.21/download"
      "https://crates-io.s3-us-west-1.amazonaws.com/crates/libc/libc-0.2.21.crate",
    ],
    type = "tar.gz",
    strip_prefix = "libc-0.2.21",
    build_file_content = """
package(default_visibility = ["//visibility:public"])

licenses(["notice"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)
rust_library(
    name = "libc",
    srcs = glob(["lib.rs", "src/**/*.rs"]),
    deps = [
    ],
    rustc_flags = [
        "--cap-lints warn",
    ],
    crate_features = [
        "default",
        "use_std",
    ],
)
""",
)

new_http_archive(
    name = "io_crates_libgit2_sys_0_6_7",
    urls = [
      # Bazel's downloader renders HTTP page instead of downloading for some reason.
      #"https://crates.io/api/v1/crates/libgit2-sys/0.6.7/download"
      "https://crates-io.s3-us-west-1.amazonaws.com/crates/libgit2-sys/libgit2-sys-0.6.7.crate",
    ],
    type = "tar.gz",
    strip_prefix = "libgit2-sys-0.6.7",
    build_file_content = """
package(default_visibility = ["//visibility:public"])

licenses(["notice"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)
rust_library(
    name = "libgit2_sys",
    srcs = glob(["lib.rs", "src/**/*.rs"]),
    deps = [
        "@io_crates_cmake_0_1_22//:cmake",
        "@io_crates_curl_sys_0_3_10//:curl_sys",
        "@io_crates_gcc_0_3_45//:gcc",
        "@io_crates_libc_0_2_21//:libc",
        "@io_crates_libssh2_sys_0_2_5//:libssh2_sys",
        "@io_crates_libz_sys_1_0_13//:libz_sys",
        "@io_crates_openssl_sys_0_9_11//:openssl_sys",
        "@io_crates_pkg_config_0_3_9//:pkg_config",
    ],
    rustc_flags = [
        "--cap-lints warn",
    ],
    crate_features = [
        "curl",
        "curl-sys",
        "https",
        "libssh2-sys",
        "openssl-sys",
        "ssh",
    ],
)
""",
)

new_http_archive(
    name = "io_crates_libssh2_sys_0_2_5",
    urls = [
      # Bazel's downloader renders HTTP page instead of downloading for some reason.
      #"https://crates.io/api/v1/crates/libssh2-sys/0.2.5/download"
      "https://crates-io.s3-us-west-1.amazonaws.com/crates/libssh2-sys/libssh2-sys-0.2.5.crate",
    ],
    type = "tar.gz",
    strip_prefix = "libssh2-sys-0.2.5",
    build_file_content = """
package(default_visibility = ["//visibility:public"])

licenses(["notice"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)
rust_library(
    name = "libssh2_sys",
    srcs = glob(["lib.rs", "src/**/*.rs"]),
    deps = [
        "@io_crates_cmake_0_1_22//:cmake",
        "@io_crates_libc_0_2_21//:libc",
        "@io_crates_libz_sys_1_0_13//:libz_sys",
        "@io_crates_openssl_sys_0_9_11//:openssl_sys",
        "@io_crates_pkg_config_0_3_9//:pkg_config",
    ],
    rustc_flags = [
        "--cap-lints warn",
    ],
    crate_features = [
    ],
)
""",
)

new_http_archive(
    name = "io_crates_libz_sys_1_0_13",
    urls = [
      # Bazel's downloader renders HTTP page instead of downloading for some reason.
      #"https://crates.io/api/v1/crates/libz-sys/1.0.13/download"
      "https://crates-io.s3-us-west-1.amazonaws.com/crates/libz-sys/libz-sys-1.0.13.crate",
    ],
    type = "tar.gz",
    strip_prefix = "libz-sys-1.0.13",
    build_file_content = """
package(default_visibility = ["//visibility:public"])

licenses(["notice"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)
rust_library(
    name = "libz_sys",
    srcs = glob(["lib.rs", "src/**/*.rs"]),
    deps = [
        "@io_crates_gcc_0_3_45//:gcc",
        "@io_crates_libc_0_2_21//:libc",
        "@io_crates_pkg_config_0_3_9//:pkg_config",
    ],
    rustc_flags = [
        "--cap-lints warn",
    ],
    crate_features = [
    ],
)
""",
)

new_http_archive(
    name = "io_crates_log_0_3_7",
    urls = [
      # Bazel's downloader renders HTTP page instead of downloading for some reason.
      #"https://crates.io/api/v1/crates/log/0.3.7/download"
      "https://crates-io.s3-us-west-1.amazonaws.com/crates/log/log-0.3.7.crate",
    ],
    type = "tar.gz",
    strip_prefix = "log-0.3.7",
    build_file_content = """
package(default_visibility = ["//visibility:public"])

licenses(["notice"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)
rust_library(
    name = "log",
    srcs = glob(["lib.rs", "src/**/*.rs"]),
    deps = [
    ],
    rustc_flags = [
        "--cap-lints warn",
    ],
    crate_features = [
        "default",
        "use_std",
    ],
)
""",
)

new_http_archive(
    name = "io_crates_matches_0_1_4",
    urls = [
      # Bazel's downloader renders HTTP page instead of downloading for some reason.
      #"https://crates.io/api/v1/crates/matches/0.1.4/download"
      "https://crates-io.s3-us-west-1.amazonaws.com/crates/matches/matches-0.1.4.crate",
    ],
    type = "tar.gz",
    strip_prefix = "matches-0.1.4",
    build_file_content = """
package(default_visibility = ["//visibility:public"])

licenses(["notice"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)
rust_library(
    name = "matches",
    srcs = glob(["lib.rs", "src/**/*.rs"]),
    deps = [
    ],
    rustc_flags = [
        "--cap-lints warn",
    ],
    crate_features = [
    ],
)
""",
)

new_http_archive(
    name = "io_crates_memchr_0_1_11",
    urls = [
      # Bazel's downloader renders HTTP page instead of downloading for some reason.
      #"https://crates.io/api/v1/crates/memchr/0.1.11/download"
      "https://crates-io.s3-us-west-1.amazonaws.com/crates/memchr/memchr-0.1.11.crate",
    ],
    type = "tar.gz",
    strip_prefix = "memchr-0.1.11",
    build_file_content = """
package(default_visibility = ["//visibility:public"])

licenses(["notice"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)
rust_library(
    name = "memchr",
    srcs = glob(["lib.rs", "src/**/*.rs"]),
    deps = [
        "@io_crates_libc_0_2_21//:libc",
    ],
    rustc_flags = [
        "--cap-lints warn",
    ],
    crate_features = [
    ],
)
""",
)

new_http_archive(
    name = "io_crates_miniz_sys_0_1_9",
    urls = [
      # Bazel's downloader renders HTTP page instead of downloading for some reason.
      #"https://crates.io/api/v1/crates/miniz-sys/0.1.9/download"
      "https://crates-io.s3-us-west-1.amazonaws.com/crates/miniz-sys/miniz-sys-0.1.9.crate",
    ],
    type = "tar.gz",
    strip_prefix = "miniz-sys-0.1.9",
    build_file_content = """
package(default_visibility = ["//visibility:public"])

licenses(["notice"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)
rust_library(
    name = "miniz_sys",
    srcs = glob(["lib.rs", "src/**/*.rs"]),
    deps = [
        "@io_crates_gcc_0_3_45//:gcc",
        "@io_crates_libc_0_2_21//:libc",
    ],
    rustc_flags = [
        "--cap-lints warn",
    ],
    crate_features = [
    ],
)
""",
)

new_http_archive(
    name = "io_crates_miow_0_1_5",
    urls = [
      # Bazel's downloader renders HTTP page instead of downloading for some reason.
      #"https://crates.io/api/v1/crates/miow/0.1.5/download"
      "https://crates-io.s3-us-west-1.amazonaws.com/crates/miow/miow-0.1.5.crate",
    ],
    type = "tar.gz",
    strip_prefix = "miow-0.1.5",
    build_file_content = """
package(default_visibility = ["//visibility:public"])

licenses(["notice"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)
rust_library(
    name = "miow",
    srcs = glob(["lib.rs", "src/**/*.rs"]),
    deps = [
        "@io_crates_kernel32_sys_0_2_2//:kernel32_sys",
        "@io_crates_net2_0_2_27//:net2",
        "@io_crates_winapi_0_2_8//:winapi",
        "@io_crates_ws2_32_sys_0_2_1//:ws2_32_sys",
    ],
    rustc_flags = [
        "--cap-lints warn",
    ],
    crate_features = [
    ],
)
""",
)

new_http_archive(
    name = "io_crates_net2_0_2_27",
    urls = [
      # Bazel's downloader renders HTTP page instead of downloading for some reason.
      #"https://crates.io/api/v1/crates/net2/0.2.27/download"
      "https://crates-io.s3-us-west-1.amazonaws.com/crates/net2/net2-0.2.27.crate",
    ],
    type = "tar.gz",
    strip_prefix = "net2-0.2.27",
    build_file_content = """
package(default_visibility = ["//visibility:public"])

licenses(["notice"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)
rust_library(
    name = "net2",
    srcs = glob(["lib.rs", "src/**/*.rs"]),
    deps = [
        "@io_crates_cfg_if_0_1_0//:cfg_if",
        "@io_crates_kernel32_sys_0_2_2//:kernel32_sys",
        "@io_crates_libc_0_2_21//:libc",
        "@io_crates_winapi_0_2_8//:winapi",
        "@io_crates_ws2_32_sys_0_2_1//:ws2_32_sys",
    ],
    rustc_flags = [
        "--cap-lints warn",
    ],
    crate_features = [
    ],
)
""",
)

new_http_archive(
    name = "io_crates_nom_2_2_1",
    urls = [
      # Bazel's downloader renders HTTP page instead of downloading for some reason.
      #"https://crates.io/api/v1/crates/nom/2.2.1/download"
      "https://crates-io.s3-us-west-1.amazonaws.com/crates/nom/nom-2.2.1.crate",
    ],
    type = "tar.gz",
    strip_prefix = "nom-2.2.1",
    build_file_content = """
package(default_visibility = ["//visibility:public"])

licenses(["notice"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)
rust_library(
    name = "nom",
    srcs = glob(["lib.rs", "src/**/*.rs"]),
    deps = [
    ],
    rustc_flags = [
        "--cap-lints warn",
    ],
    crate_features = [
        "default",
        "stream",
    ],
)
""",
)

new_http_archive(
    name = "io_crates_num_cpus_1_3_0",
    urls = [
      # Bazel's downloader renders HTTP page instead of downloading for some reason.
      #"https://crates.io/api/v1/crates/num_cpus/1.3.0/download"
      "https://crates-io.s3-us-west-1.amazonaws.com/crates/num_cpus/num_cpus-1.3.0.crate",
    ],
    type = "tar.gz",
    strip_prefix = "num_cpus-1.3.0",
    build_file_content = """
package(default_visibility = ["//visibility:public"])

licenses(["notice"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)
rust_library(
    name = "num_cpus",
    srcs = glob(["lib.rs", "src/**/*.rs"]),
    deps = [
        "@io_crates_libc_0_2_21//:libc",
    ],
    rustc_flags = [
        "--cap-lints warn",
    ],
    crate_features = [
    ],
)
""",
)

new_http_archive(
    name = "io_crates_openssl_0_9_11",
    urls = [
      # Bazel's downloader renders HTTP page instead of downloading for some reason.
      #"https://crates.io/api/v1/crates/openssl/0.9.11/download"
      "https://crates-io.s3-us-west-1.amazonaws.com/crates/openssl/openssl-0.9.11.crate",
    ],
    type = "tar.gz",
    strip_prefix = "openssl-0.9.11",
    build_file_content = """
package(default_visibility = ["//visibility:public"])

licenses(["notice"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)
rust_library(
    name = "openssl",
    srcs = glob(["lib.rs", "src/**/*.rs"]),
    deps = [
        "@io_crates_bitflags_0_8_2//:bitflags",
        "@io_crates_foreign_types_0_2_0//:foreign_types",
        "@io_crates_lazy_static_0_2_8//:lazy_static",
        "@io_crates_libc_0_2_21//:libc",
        "@io_crates_openssl_sys_0_9_11//:openssl_sys",
    ],
    rustc_flags = [
        "--cap-lints warn",
    ],
    crate_features = [
    ],
)
""",
)

new_http_archive(
    name = "io_crates_openssl_probe_0_1_1",
    urls = [
      # Bazel's downloader renders HTTP page instead of downloading for some reason.
      #"https://crates.io/api/v1/crates/openssl-probe/0.1.1/download"
      "https://crates-io.s3-us-west-1.amazonaws.com/crates/openssl-probe/openssl-probe-0.1.1.crate",
    ],
    type = "tar.gz",
    strip_prefix = "openssl-probe-0.1.1",
    build_file_content = """
package(default_visibility = ["//visibility:public"])

licenses(["notice"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)
rust_library(
    name = "openssl_probe",
    srcs = glob(["lib.rs", "src/**/*.rs"]),
    deps = [
    ],
    rustc_flags = [
        "--cap-lints warn",
    ],
    crate_features = [
    ],
)
""",
)

new_http_archive(
    name = "io_crates_openssl_sys_0_9_11",
    urls = [
      # Bazel's downloader renders HTTP page instead of downloading for some reason.
      #"https://crates.io/api/v1/crates/openssl-sys/0.9.11/download"
      "https://crates-io.s3-us-west-1.amazonaws.com/crates/openssl-sys/openssl-sys-0.9.11.crate",
    ],
    type = "tar.gz",
    strip_prefix = "openssl-sys-0.9.11",
    build_file_content = """
package(default_visibility = ["//visibility:public"])

licenses(["notice"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)
rust_library(
    name = "openssl_sys",
    srcs = glob(["lib.rs", "src/**/*.rs"]),
    deps = [
        "@io_crates_gcc_0_3_45//:gcc",
        "@io_crates_gdi32_sys_0_2_0//:gdi32_sys",
        "@io_crates_libc_0_2_21//:libc",
        "@io_crates_pkg_config_0_3_9//:pkg_config",
        "@io_crates_user32_sys_0_2_0//:user32_sys",
    ],
    rustc_flags = [
        "--cap-lints warn",
    ],
    crate_features = [
    ],
)
""",
)

new_http_archive(
    name = "io_crates_pkg_config_0_3_9",
    urls = [
      # Bazel's downloader renders HTTP page instead of downloading for some reason.
      #"https://crates.io/api/v1/crates/pkg-config/0.3.9/download"
      "https://crates-io.s3-us-west-1.amazonaws.com/crates/pkg-config/pkg-config-0.3.9.crate",
    ],
    type = "tar.gz",
    strip_prefix = "pkg-config-0.3.9",
    build_file_content = """
package(default_visibility = ["//visibility:public"])

licenses(["notice"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)
rust_library(
    name = "pkg_config",
    srcs = glob(["lib.rs", "src/**/*.rs"]),
    deps = [
    ],
    rustc_flags = [
        "--cap-lints warn",
    ],
    crate_features = [
    ],
)
""",
)

new_http_archive(
    name = "io_crates_psapi_sys_0_1_0",
    urls = [
      # Bazel's downloader renders HTTP page instead of downloading for some reason.
      #"https://crates.io/api/v1/crates/psapi-sys/0.1.0/download"
      "https://crates-io.s3-us-west-1.amazonaws.com/crates/psapi-sys/psapi-sys-0.1.0.crate",
    ],
    type = "tar.gz",
    strip_prefix = "psapi-sys-0.1.0",
    build_file_content = """
package(default_visibility = ["//visibility:public"])

licenses(["notice"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)
rust_library(
    name = "psapi_sys",
    srcs = glob(["lib.rs", "src/**/*.rs"]),
    deps = [
        "@io_crates_winapi_0_2_8//:winapi",
        "@io_crates_winapi_build_0_1_1//:winapi_build",
    ],
    rustc_flags = [
        "--cap-lints warn",
    ],
    crate_features = [
    ],
)
""",
)

new_http_archive(
    name = "io_crates_rand_0_3_15",
    urls = [
      # Bazel's downloader renders HTTP page instead of downloading for some reason.
      #"https://crates.io/api/v1/crates/rand/0.3.15/download"
      "https://crates-io.s3-us-west-1.amazonaws.com/crates/rand/rand-0.3.15.crate",
    ],
    type = "tar.gz",
    strip_prefix = "rand-0.3.15",
    build_file_content = """
package(default_visibility = ["//visibility:public"])

licenses(["notice"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)
rust_library(
    name = "rand",
    srcs = glob(["lib.rs", "src/**/*.rs"]),
    deps = [
        "@io_crates_libc_0_2_21//:libc",
    ],
    rustc_flags = [
        "--cap-lints warn",
    ],
    crate_features = [
    ],
)
""",
)

new_http_archive(
    name = "io_crates_regex_0_1_80",
    urls = [
      # Bazel's downloader renders HTTP page instead of downloading for some reason.
      #"https://crates.io/api/v1/crates/regex/0.1.80/download"
      "https://crates-io.s3-us-west-1.amazonaws.com/crates/regex/regex-0.1.80.crate",
    ],
    type = "tar.gz",
    strip_prefix = "regex-0.1.80",
    build_file_content = """
package(default_visibility = ["//visibility:public"])

licenses(["notice"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)
rust_library(
    name = "regex",
    srcs = glob(["lib.rs", "src/**/*.rs"]),
    deps = [
        "@io_crates_aho_corasick_0_5_3//:aho_corasick",
        "@io_crates_memchr_0_1_11//:memchr",
        "@io_crates_regex_syntax_0_3_9//:regex_syntax",
        "@io_crates_thread_local_0_2_7//:thread_local",
        "@io_crates_utf8_ranges_0_1_3//:utf8_ranges",
    ],
    rustc_flags = [
        "--cap-lints warn",
    ],
    crate_features = [
    ],
)
""",
)

new_http_archive(
    name = "io_crates_regex_syntax_0_3_9",
    urls = [
      # Bazel's downloader renders HTTP page instead of downloading for some reason.
      #"https://crates.io/api/v1/crates/regex-syntax/0.3.9/download"
      "https://crates-io.s3-us-west-1.amazonaws.com/crates/regex-syntax/regex-syntax-0.3.9.crate",
    ],
    type = "tar.gz",
    strip_prefix = "regex-syntax-0.3.9",
    build_file_content = """
package(default_visibility = ["//visibility:public"])

licenses(["notice"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)
rust_library(
    name = "regex_syntax",
    srcs = glob(["lib.rs", "src/**/*.rs"]),
    deps = [
    ],
    rustc_flags = [
        "--cap-lints warn",
    ],
    crate_features = [
    ],
)
""",
)

new_http_archive(
    name = "io_crates_rustc_serialize_0_3_24",
    urls = [
      # Bazel's downloader renders HTTP page instead of downloading for some reason.
      #"https://crates.io/api/v1/crates/rustc-serialize/0.3.24/download"
      "https://crates-io.s3-us-west-1.amazonaws.com/crates/rustc-serialize/rustc-serialize-0.3.24.crate",
    ],
    type = "tar.gz",
    strip_prefix = "rustc-serialize-0.3.24",
    build_file_content = """
package(default_visibility = ["//visibility:public"])

licenses(["notice"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)
rust_library(
    name = "rustc_serialize",
    srcs = glob(["lib.rs", "src/**/*.rs"]),
    deps = [
    ],
    rustc_flags = [
        "--cap-lints warn",
    ],
    crate_features = [
    ],
)
""",
)

new_http_archive(
    name = "io_crates_semver_0_5_1",
    urls = [
      # Bazel's downloader renders HTTP page instead of downloading for some reason.
      #"https://crates.io/api/v1/crates/semver/0.5.1/download"
      "https://crates-io.s3-us-west-1.amazonaws.com/crates/semver/semver-0.5.1.crate",
    ],
    type = "tar.gz",
    strip_prefix = "semver-0.5.1",
    build_file_content = """
package(default_visibility = ["//visibility:public"])

licenses(["notice"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)
rust_library(
    name = "semver",
    srcs = glob(["lib.rs", "src/**/*.rs"]),
    deps = [
        "@io_crates_semver_parser_0_6_2//:semver_parser",
    ],
    rustc_flags = [
        "--cap-lints warn",
    ],
    crate_features = [
        "default",
    ],
)
""",
)

new_http_archive(
    name = "io_crates_semver_parser_0_6_2",
    urls = [
      # Bazel's downloader renders HTTP page instead of downloading for some reason.
      #"https://crates.io/api/v1/crates/semver-parser/0.6.2/download"
      "https://crates-io.s3-us-west-1.amazonaws.com/crates/semver-parser/semver-parser-0.6.2.crate",
    ],
    type = "tar.gz",
    strip_prefix = "semver-parser-0.6.2",
    build_file_content = """
package(default_visibility = ["//visibility:public"])

licenses(["notice"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)
rust_library(
    name = "semver_parser",
    srcs = glob(["lib.rs", "src/**/*.rs"]),
    deps = [
        "@io_crates_lazy_static_0_2_8//:lazy_static",
        "@io_crates_regex_0_1_80//:regex",
    ],
    rustc_flags = [
        "--cap-lints warn",
    ],
    crate_features = [
    ],
)
""",
)

new_http_archive(
    name = "io_crates_shell_escape_0_1_3",
    urls = [
      # Bazel's downloader renders HTTP page instead of downloading for some reason.
      #"https://crates.io/api/v1/crates/shell-escape/0.1.3/download"
      "https://crates-io.s3-us-west-1.amazonaws.com/crates/shell-escape/shell-escape-0.1.3.crate",
    ],
    type = "tar.gz",
    strip_prefix = "shell-escape-0.1.3",
    build_file_content = """
package(default_visibility = ["//visibility:public"])

licenses(["notice"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)
rust_library(
    name = "shell_escape",
    srcs = glob(["lib.rs", "src/**/*.rs"]),
    deps = [
    ],
    rustc_flags = [
        "--cap-lints warn",
    ],
    crate_features = [
    ],
)
""",
)

new_http_archive(
    name = "io_crates_strsim_0_5_2",
    urls = [
      # Bazel's downloader renders HTTP page instead of downloading for some reason.
      #"https://crates.io/api/v1/crates/strsim/0.5.2/download"
      "https://crates-io.s3-us-west-1.amazonaws.com/crates/strsim/strsim-0.5.2.crate",
    ],
    type = "tar.gz",
    strip_prefix = "strsim-0.5.2",
    build_file_content = """
package(default_visibility = ["//visibility:public"])

licenses(["notice"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)
rust_library(
    name = "strsim",
    srcs = glob(["lib.rs", "src/**/*.rs"]),
    deps = [
    ],
    rustc_flags = [
        "--cap-lints warn",
    ],
    crate_features = [
    ],
)
""",
)

new_http_archive(
    name = "io_crates_tar_0_4_11",
    urls = [
      # Bazel's downloader renders HTTP page instead of downloading for some reason.
      #"https://crates.io/api/v1/crates/tar/0.4.11/download"
      "https://crates-io.s3-us-west-1.amazonaws.com/crates/tar/tar-0.4.11.crate",
    ],
    type = "tar.gz",
    strip_prefix = "tar-0.4.11",
    build_file_content = """
package(default_visibility = ["//visibility:public"])

licenses(["notice"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)
rust_library(
    name = "tar",
    srcs = glob(["lib.rs", "src/**/*.rs"]),
    deps = [
        "@io_crates_filetime_0_1_10//:filetime",
        "@io_crates_libc_0_2_21//:libc",
    ],
    rustc_flags = [
        "--cap-lints warn",
    ],
    crate_features = [
    ],
)
""",
)

new_http_archive(
    name = "io_crates_tempdir_0_3_5",
    urls = [
      # Bazel's downloader renders HTTP page instead of downloading for some reason.
      #"https://crates.io/api/v1/crates/tempdir/0.3.5/download"
      "https://crates-io.s3-us-west-1.amazonaws.com/crates/tempdir/tempdir-0.3.5.crate",
    ],
    type = "tar.gz",
    strip_prefix = "tempdir-0.3.5",
    build_file_content = """
package(default_visibility = ["//visibility:public"])

licenses(["notice"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)
rust_library(
    name = "tempdir",
    srcs = glob(["lib.rs", "src/**/*.rs"]),
    deps = [
        "@io_crates_rand_0_3_15//:rand",
    ],
    rustc_flags = [
        "--cap-lints warn",
    ],
    crate_features = [
    ],
)
""",
)

new_http_archive(
    name = "io_crates_term_0_4_5",
    urls = [
      # Bazel's downloader renders HTTP page instead of downloading for some reason.
      #"https://crates.io/api/v1/crates/term/0.4.5/download"
      "https://crates-io.s3-us-west-1.amazonaws.com/crates/term/term-0.4.5.crate",
    ],
    type = "tar.gz",
    strip_prefix = "term-0.4.5",
    build_file_content = """
package(default_visibility = ["//visibility:public"])

licenses(["notice"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)
rust_library(
    name = "term",
    srcs = glob(["lib.rs", "src/**/*.rs"]),
    deps = [
        "@io_crates_kernel32_sys_0_2_2//:kernel32_sys",
        "@io_crates_winapi_0_2_8//:winapi",
    ],
    rustc_flags = [
        "--cap-lints warn",
    ],
    crate_features = [
        "default",
    ],
)
""",
)

new_http_archive(
    name = "io_crates_thread_id_2_0_0",
    urls = [
      # Bazel's downloader renders HTTP page instead of downloading for some reason.
      #"https://crates.io/api/v1/crates/thread-id/2.0.0/download"
      "https://crates-io.s3-us-west-1.amazonaws.com/crates/thread-id/thread-id-2.0.0.crate",
    ],
    type = "tar.gz",
    strip_prefix = "thread-id-2.0.0",
    build_file_content = """
package(default_visibility = ["//visibility:public"])

licenses(["notice"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)
rust_library(
    name = "thread_id",
    srcs = glob(["lib.rs", "src/**/*.rs"]),
    deps = [
        "@io_crates_kernel32_sys_0_2_2//:kernel32_sys",
        "@io_crates_libc_0_2_21//:libc",
    ],
    rustc_flags = [
        "--cap-lints warn",
    ],
    crate_features = [
    ],
)
""",
)

new_http_archive(
    name = "io_crates_thread_local_0_2_7",
    urls = [
      # Bazel's downloader renders HTTP page instead of downloading for some reason.
      #"https://crates.io/api/v1/crates/thread_local/0.2.7/download"
      "https://crates-io.s3-us-west-1.amazonaws.com/crates/thread_local/thread_local-0.2.7.crate",
    ],
    type = "tar.gz",
    strip_prefix = "thread_local-0.2.7",
    build_file_content = """
package(default_visibility = ["//visibility:public"])

licenses(["notice"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)
rust_library(
    name = "thread_local",
    srcs = glob(["lib.rs", "src/**/*.rs"]),
    deps = [
        "@io_crates_thread_id_2_0_0//:thread_id",
    ],
    rustc_flags = [
        "--cap-lints warn",
    ],
    crate_features = [
    ],
)
""",
)

new_http_archive(
    name = "io_crates_toml_0_2_1",
    urls = [
      # Bazel's downloader renders HTTP page instead of downloading for some reason.
      #"https://crates.io/api/v1/crates/toml/0.2.1/download"
      "https://crates-io.s3-us-west-1.amazonaws.com/crates/toml/toml-0.2.1.crate",
    ],
    type = "tar.gz",
    strip_prefix = "toml-0.2.1",
    build_file_content = """
package(default_visibility = ["//visibility:public"])

licenses(["notice"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)
rust_library(
    name = "toml",
    srcs = glob(["lib.rs", "src/**/*.rs"]),
    deps = [
        "@io_crates_rustc_serialize_0_3_24//:rustc_serialize",
    ],
    rustc_flags = [
        "--cap-lints warn",
    ],
    crate_features = [
        "default",
        "rustc-serialize",
    ],
)
""",
)

new_http_archive(
    name = "io_crates_unicode_bidi_0_2_5",
    urls = [
      # Bazel's downloader renders HTTP page instead of downloading for some reason.
      #"https://crates.io/api/v1/crates/unicode-bidi/0.2.5/download"
      "https://crates-io.s3-us-west-1.amazonaws.com/crates/unicode-bidi/unicode-bidi-0.2.5.crate",
    ],
    type = "tar.gz",
    strip_prefix = "unicode-bidi-0.2.5",
    build_file_content = """
package(default_visibility = ["//visibility:public"])

licenses(["notice"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)
rust_library(
    name = "unicode_bidi",
    srcs = glob(["lib.rs", "src/**/*.rs"]),
    deps = [
        "@io_crates_matches_0_1_4//:matches",
    ],
    rustc_flags = [
        "--cap-lints warn",
    ],
    crate_features = [
    ],
)
""",
)

new_http_archive(
    name = "io_crates_unicode_normalization_0_1_4",
    urls = [
      # Bazel's downloader renders HTTP page instead of downloading for some reason.
      #"https://crates.io/api/v1/crates/unicode-normalization/0.1.4/download"
      "https://crates-io.s3-us-west-1.amazonaws.com/crates/unicode-normalization/unicode-normalization-0.1.4.crate",
    ],
    type = "tar.gz",
    strip_prefix = "unicode-normalization-0.1.4",
    build_file_content = """
package(default_visibility = ["//visibility:public"])

licenses(["notice"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)
rust_library(
    name = "unicode_normalization",
    srcs = glob(["lib.rs", "src/**/*.rs"]),
    deps = [
    ],
    rustc_flags = [
        "--cap-lints warn",
    ],
    crate_features = [
    ],
)
""",
)

new_http_archive(
    name = "io_crates_url_1_4_0",
    urls = [
      # Bazel's downloader renders HTTP page instead of downloading for some reason.
      #"https://crates.io/api/v1/crates/url/1.4.0/download"
      "https://crates-io.s3-us-west-1.amazonaws.com/crates/url/url-1.4.0.crate",
    ],
    type = "tar.gz",
    strip_prefix = "url-1.4.0",
    build_file_content = """
package(default_visibility = ["//visibility:public"])

licenses(["notice"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)
rust_library(
    name = "url",
    srcs = glob(["lib.rs", "src/**/*.rs"]),
    deps = [
        "@io_crates_idna_0_1_1//:idna",
        "@io_crates_matches_0_1_4//:matches",
    ],
    rustc_flags = [
        "--cap-lints warn",
    ],
    crate_features = [
    ],
)
""",
)

new_http_archive(
    name = "io_crates_user32_sys_0_2_0",
    urls = [
      # Bazel's downloader renders HTTP page instead of downloading for some reason.
      #"https://crates.io/api/v1/crates/user32-sys/0.2.0/download"
      "https://crates-io.s3-us-west-1.amazonaws.com/crates/user32-sys/user32-sys-0.2.0.crate",
    ],
    type = "tar.gz",
    strip_prefix = "user32-sys-0.2.0",
    build_file_content = """
package(default_visibility = ["//visibility:public"])

licenses(["notice"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)
rust_library(
    name = "user32_sys",
    srcs = glob(["lib.rs", "src/**/*.rs"]),
    deps = [
        "@io_crates_winapi_0_2_8//:winapi",
        "@io_crates_winapi_build_0_1_1//:winapi_build",
    ],
    rustc_flags = [
        "--cap-lints warn",
    ],
    crate_features = [
    ],
)
""",
)

new_http_archive(
    name = "io_crates_utf8_ranges_0_1_3",
    urls = [
      # Bazel's downloader renders HTTP page instead of downloading for some reason.
      #"https://crates.io/api/v1/crates/utf8-ranges/0.1.3/download"
      "https://crates-io.s3-us-west-1.amazonaws.com/crates/utf8-ranges/utf8-ranges-0.1.3.crate",
    ],
    type = "tar.gz",
    strip_prefix = "utf8-ranges-0.1.3",
    build_file_content = """
package(default_visibility = ["//visibility:public"])

licenses(["notice"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)
rust_library(
    name = "utf8_ranges",
    srcs = glob(["lib.rs", "src/**/*.rs"]),
    deps = [
    ],
    rustc_flags = [
        "--cap-lints warn",
    ],
    crate_features = [
    ],
)
""",
)

new_http_archive(
    name = "io_crates_winapi_0_2_8",
    urls = [
      # Bazel's downloader renders HTTP page instead of downloading for some reason.
      #"https://crates.io/api/v1/crates/winapi/0.2.8/download"
      "https://crates-io.s3-us-west-1.amazonaws.com/crates/winapi/winapi-0.2.8.crate",
    ],
    type = "tar.gz",
    strip_prefix = "winapi-0.2.8",
    build_file_content = """
package(default_visibility = ["//visibility:public"])

licenses(["notice"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)
rust_library(
    name = "winapi",
    srcs = glob(["lib.rs", "src/**/*.rs"]),
    deps = [
    ],
    rustc_flags = [
        "--cap-lints warn",
    ],
    crate_features = [
    ],
)
""",
)

new_http_archive(
    name = "io_crates_winapi_build_0_1_1",
    urls = [
      # Bazel's downloader renders HTTP page instead of downloading for some reason.
      #"https://crates.io/api/v1/crates/winapi-build/0.1.1/download"
      "https://crates-io.s3-us-west-1.amazonaws.com/crates/winapi-build/winapi-build-0.1.1.crate",
    ],
    type = "tar.gz",
    strip_prefix = "winapi-build-0.1.1",
    build_file_content = """
package(default_visibility = ["//visibility:public"])

licenses(["notice"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)
rust_library(
    name = "winapi_build",
    srcs = glob(["lib.rs", "src/**/*.rs"]),
    deps = [
    ],
    rustc_flags = [
        "--cap-lints warn",
    ],
    crate_features = [
    ],
)
""",
)

new_http_archive(
    name = "io_crates_ws2_32_sys_0_2_1",
    urls = [
      # Bazel's downloader renders HTTP page instead of downloading for some reason.
      #"https://crates.io/api/v1/crates/ws2_32-sys/0.2.1/download"
      "https://crates-io.s3-us-west-1.amazonaws.com/crates/ws2_32-sys/ws2_32-sys-0.2.1.crate",
    ],
    type = "tar.gz",
    strip_prefix = "ws2_32-sys-0.2.1",
    build_file_content = """
package(default_visibility = ["//visibility:public"])

licenses(["notice"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)
rust_library(
    name = "ws2_32_sys",
    srcs = glob(["lib.rs", "src/**/*.rs"]),
    deps = [
        "@io_crates_winapi_0_2_8//:winapi",
        "@io_crates_winapi_build_0_1_1//:winapi_build",
    ],
    rustc_flags = [
        "--cap-lints warn",
    ],
    crate_features = [
    ],
)
""",
)
