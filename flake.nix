{
  description = "Your trusty omnibox search.";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    home-manager.url = "github:nix-community/home-manager";
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      crane,
      home-manager,
    }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };

      devInputs = with pkgs; [
        rustc
        rustfmt
        cargo
      ];

      nativeBuildInputs =
        with pkgs;
        [
          # cmake pkgconf
          makeWrapper
        ];

      buildInputs = with pkgs; [ ];

      cargoTOML = builtins.fromTOML (builtins.readFile (./. + "/Cargo.toml"));

      inherit (cargoTOML.workspace.package) version;
      pname = "centerpiece";

      craneLib = crane.lib.${system};
      assetFilter = path: _type: builtins.match ".*ttf$" path != null;
      assetOrCargo =
        path: type: (assetFilter path type) || (craneLib.filterCargoSources path type);
      commonArgs = {
        src = pkgs.lib.cleanSourceWith {
          src = craneLib.path ./.;
          filter = assetOrCargo;
        };
        inherit pname version;
      };
      cargoArtifacts = craneLib.buildDepsOnly commonArgs;
      cargoClippy = craneLib.cargoClippy (
        commonArgs
        // {
          inherit cargoArtifacts;
          cargoClippyExtraArgs = "--all-targets --all-features";
        }
      );
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        inherit nativeBuildInputs buildInputs;
        packages = devInputs;
        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
          pkgs.wayland
          pkgs.libxkbcommon
          pkgs.vulkan-loader
          pkgs.libGL
        ];
      };
      packages.${system} = {
        default = craneLib.buildPackage (
          commonArgs
          // {
            inherit
              cargoArtifacts
              nativeBuildInputs
              buildInputs
              pname
            ;
            postInstall = ''
              wrapProgram "$out/bin/${pname}" \
                --prefix LD_LIBRARY_PATH : ${
                  pkgs.lib.makeLibraryPath [
                    pkgs.wayland
                    pkgs.libxkbcommon
                    pkgs.vulkan-loader
                    pkgs.libGL
                  ]
                }
            '';
            meta = with pkgs.lib; {
              description = "Your trusty omnibox search.";
              homepage = "https://github.com/friedow/centerpiece";
              platforms = platforms.linux;
              license = licenses.mit;
              maintainers = [ "friedow" ];
            };
          }
        );
        index-git-repositories = craneLib.buildPackage (
          commonArgs
          // rec {
            inherit cargoArtifacts;
            pname = "index-git-repositories";
            cargoExtraArgs = "-p ${pname}";
            meta.mainProgram = pname;
          }
        );
      };
      checks.${system} = {
        inherit (self.outputs.packages.${system}) default index-git-repositories;
        shell = self.outputs.devShells.${system}.default;
        inherit cargoClippy;
      };
      hmModules.${system}.default = import ./home-manager-module.nix {
        centerpiece = self.outputs.packages.${system}.default;
        inherit (self.outputs.packages.${system}) index-git-repositories;
      };
    };
}
