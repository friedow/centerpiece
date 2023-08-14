{
  description = "Your trusty omnibox search.";

  inputs = { nixpkgs.url = "github:NixOS/nixpkgs"; };

  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };

      devInputs = with pkgs; [ rustc rustfmt cargo ];

      nativeBuildInputs = with pkgs; [ cmake pkgconf makeWrapper ];

      buildInputs = with pkgs; [
        wayland

        freetype
        expat
        libGL
        libglvnd
        fontconfig
        libxkbcommon

        xorg.libX11
        xorg.libXcursor
        xorg.libXi
        xorg.libXrandr
        xorg.libxcb
      ];
    in {
      devShells.${system}.default = pkgs.mkShell {
        inherit nativeBuildInputs buildInputs;
        packages = devInputs;
        LD_LIBRARY_PATH =
          pkgs.lib.makeLibraryPath [ pkgs.vulkan-loader pkgs.libGL ];
      };

      packages.${system}.default = pkgs.rustPlatform.buildRustPackage rec {
        pname = "centerpiece";
        version = "0.1.0";

        inherit nativeBuildInputs buildInputs;

        postInstall = ''
          wrapProgram "$out/bin/${pname}" \
            --prefix LD_LIBRARY_PATH : ${
              pkgs.lib.makeLibraryPath [ pkgs.vulkan-loader pkgs.libGL ]
            } \
            --suffix XDG_DATA_DIRS : "${pkgs.papirus-icon-theme}/share"
        '';

        src = ./.;

        cargoLock.lockFile = ./Cargo.lock;

        meta = with pkgs.lib; {
          description = "Your trusty omnibox search.";
          homepage = "https://github.com/friedow/centerpiece";
          platforms = platforms.linux;
          license = licenses.mit;
          maintainers = [ "friedow" ];
        };
      };
    };
}
