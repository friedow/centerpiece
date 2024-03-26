{nixpkgs ? import <nixpkgs> {}}:
let
  owner = "edolstra";
  repo = "flake-compat";
  rev = "b7547d3eed6f32d06102ead8991ec52ab0a4f1a7";
  narHash = "sha256-4jY7RCWUoZ9cKD8co0/4tFARpWB+57+r1bLLvXNJliY=";

  flake-compat = import (builtins.fetchTarball {
    url = "https://github.com/${owner}/${repo}/archive/${rev}.tar.gz";
    sha256 = narHash;
  }) {src = ./.;};
  flakePackages = flake-compat.defaultNix.packages.${nixpkgs.stdenv.system};
in
  flakePackages
