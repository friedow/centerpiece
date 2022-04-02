{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
  buildInputs = [
    pkgs.pkg-config
    pkgs.go
    pkgs.gtk3
  ];
}
