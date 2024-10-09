{
  description = "Your trusty omnibox search.";

  nixConfig = {
    extra-substituters = [ "https://friedow.cachix.org" ];
    extra-trusted-public-keys = [ "friedow.cachix.org-1:JDEaYMqNgGu+bVPOca7Zu4Cp8QDMkvQpArKuwPKa29A=" ];
  };

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    home-manager.url = "github:nix-community/home-manager";
    treefmt-nix.url = "github:numtide/treefmt-nix/";
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = args: import ./nix args;

  #     packages.${system} = {
  #       default = craneLib.buildPackage (
  #         commonArgs
  #         // {
  #           inherit
  #             cargoArtifacts
  #             nativeBuildInputs
  #             buildInputs
  #             pname
  #             GIT_REV
  #             GIT_DATE
  #             ;
  #           postFixup = pkgs.lib.optional pkgs.stdenv.isLinux ''
  #             rpath=$(patchelf --print-rpath $out/bin/${pname})
  #             patchelf --set-rpath "$rpath:${libPath}" $out/bin/${pname}
  #           '';
  #         }
  #       );
  #     checks.${system} = {
  #       inherit (self.outputs.packages.${system}) default index-git-repositories;
  #       shell = self.outputs.devShells.${system}.default;
  #       inherit cargoClippy;
  #       hmModule =
  #         (nixpkgs.lib.nixosSystem {
  #           system = "x86_64-linux";
  #           modules = [
  #             home-manager.nixosModules.home-manager
  #             {
  #               home-manager.users.alice = {
  #                 imports = [ self.outputs.hmModules."x86_64-linux".default ];
  #                 programs.centerpiece = {
  #                   enable = true;
  #                   config.plugin.git_repositories.commands = [ [ "alacritty" ] ];
  #                   services.index-git-repositories = {
  #                     enable = true;
  #                     interval = "3hours";
  #                   };
  #                 };
  #                 home.stateVersion = "23.11";
  #               };
  #               users.users.alice = {
  #                 isNormalUser = true;
  #                 uid = 1000;
  #                 home = "/home/alice";
  #               };
  #             }
  #           ];
  #         }).config.system.build.vm;
  #     };
  #     hmModules.${system}.default = import ./home-manager-module.nix {
  #       centerpiece = self.outputs.packages.${system}.default;
  #       inherit (self.outputs.packages.${system}) index-git-repositories;
  #     };
  #   };
}
