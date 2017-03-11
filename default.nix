with (import <nixpkgs> {});
let
  pkgs = import <nixpkgs> {};
in pkgs.stdenv.mkDerivation rec {
  name = "raze";
  buildInputs = [ pkgs.scala ];
  propagatedBuildInputs = [ pkgs.pkgconfig pkgs.openssl pkgs.zlib ];
  shellHook = ''
    # Allow my shell to add custom snippet
    export IS_NIX_SHELL=1
    export BAZEL_SH=/run/current-system/sw/bin/bash
  '';
}
