{
  description = "Your trusty omnibox search.";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    home-manager.url = "github:nix-community/home-manager";
    treefmt-nix.url = "github:numtide/treefmt-nix/";
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, crane, treefmt-nix, home-manager, ... }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };

      devInputs = with pkgs; [ rustc rustfmt cargo ];

      nativeBuildInputs = with pkgs; [
        makeWrapper
        # wifi plugin
        # cargo networkmanager dependency
        pkgconf
        dbus
      ];

      buildInputs = with pkgs; [ dbus ];

      cargoTOML = builtins.fromTOML (builtins.readFile (./. + "/Cargo.toml"));

      inherit (cargoTOML.workspace.package) version;
      pname = "centerpiece";

      craneLib = crane.lib.${system};
      assetFilter = path: _type: builtins.match ".*ttf$" path != null;
      assetOrCargo = path: type:
        (assetFilter path type) || (craneLib.filterCargoSources path type);
      commonArgs = {
        src = pkgs.lib.cleanSourceWith {
          src = craneLib.path ./.;
          filter = assetOrCargo;
        };
        inherit pname version buildInputs nativeBuildInputs;
      };
      cargoArtifacts = craneLib.buildDepsOnly commonArgs;
      cargoClippy = craneLib.cargoClippy (commonArgs // {
        inherit cargoArtifacts;
        cargoClippyExtraArgs = "--all-targets --all-features";
      });
      GIT_DATE = "${builtins.substring 0 4 self.lastModifiedDate}-${
          builtins.substring 4 2 self.lastModifiedDate
        }-${builtins.substring 6 2 self.lastModifiedDate}";
      GIT_REV = self.shortRev or "Not committed yet.";
      treefmt = (treefmt-nix.lib.evalModule pkgs ./formatter.nix).config.build;
    in {
      devShells.${system}.default = pkgs.mkShell {
        inherit nativeBuildInputs buildInputs GIT_DATE GIT_REV;
        packages = devInputs ++ [ treefmt.wrapper ];
        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
          pkgs.wayland
          pkgs.libxkbcommon
          pkgs.vulkan-loader
          pkgs.libGL
        ];
      };
      packages.${system} = {
        default = craneLib.buildPackage (commonArgs // {
          inherit cargoArtifacts nativeBuildInputs buildInputs pname GIT_REV
            GIT_DATE;
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
            mainProgram = pname;
            maintainers = [ "friedow" ];
          };
        });
        index-git-repositories = craneLib.buildPackage (commonArgs // rec {
          inherit cargoArtifacts;
          pname = "index-git-repositories";
          cargoExtraArgs = "-p ${pname}";
          meta.mainProgram = pname;
        });
      };
      checks.${system} = {
        inherit (self.outputs.packages.${system})
          default index-git-repositories;
        shell = self.outputs.devShells.${system}.default;
        treefmt = treefmt.check self;
        inherit cargoClippy;
        hmModule = (nixpkgs.lib.nixosSystem {
          system = "x86_64-linux";
          modules = [
            home-manager.nixosModules.home-manager
            {
              home-manager.users.alice = {
                imports = [ self.outputs.hmModules."x86_64-linux".default ];
                programs.centerpiece = {
                  enable = true;
                  config.plugin.git_repositories.commands = [ [ "alacritty" ] ];
                  services.index-git-repositories = {
                    enable = true;
                    interval = "3hours";
                  };
                };
                home.stateVersion = "23.11";
              };
              users.users.alice = {
                isNormalUser = true;
                uid = 1000;
                home = "/home/alice";
              };
            }
          ];
        }).config.system.build.vm;
      };
      hmModules.${system}.default = import ./home-manager-module.nix {
        centerpiece = self.outputs.packages.${system}.default;
        inherit (self.outputs.packages.${system}) index-git-repositories;
      };
      formatter.${system} = treefmt.wrapper;
    };
}
