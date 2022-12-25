{
  description = "Your trusty omnibox search.";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/release-22.11";
  };

  outputs = { self, nixpkgs }: {

    

    packages.x86_64-linux.search = 
    let
      pkgs = import nixpkgs { system = "x86_64-linux"; };

      pname = "search-friedow-com";
      version = "0.2.0";
      src = ./.;

      frontend-build = pkgs.buildNpmPackage {
        inherit version src;
        pname = "search-friedow-com-ui";

        npmDepsHash = "sha256-kxwGqfPUmZL3tok8CXedlHk5Q3sTlraxttOwks94jo4=";

        buildPhase = ''
          export HOME=$(mktemp -d)
          npm run build

          cp -r dist $out
        '';

        distPhase = "true";
        dontInstall = true;
      };
    in
    pkgs.rustPlatform.buildRustPackage rec {
      inherit version pname;

      src = ./src-tauri;

      # sourceRoot = ./src-tauri;
      # cargoLock = { lockFile = "${src}/src-tauri/Cargo.lock"; };

      cargoSha256 = "sha256-0QruekSg2BTdM9Ds41fuLMuh0Vwy8TEGGgpAmmuI5cw=";

      # Copy the frontend static resources to final build directory
      # Also modify tauri.conf.json so that it expects the resources at the new location
      postPatch = ''
        mkdir -p frontend-build
        cp -R ${frontend-build} frontend-build
        substituteInPlace tauri.conf.json --replace '"distDir": "../dist"' '"distDir": "frontend-build"'
      '';

      # dontUnpack = true;

      buildInputs = with pkgs; [ dbus openssl webkitgtk ];
      nativeBuildInputs = with pkgs; [ pkg-config ];

      # Skip one test that fails ( tries to mutate the parent directory )
      checkFlags = [ "--skip=test_file_operation" ];

      # Rename the executable
      postInstall = ''
        mv target/release/search-friedow-com $out/bin/search-friedow-com
      '';
    };

    # packages.x86_64-linux = {
    #   search = with import nixpkgs { system = "x86_64-linux"; }; stdenv.mkDerivation {
    #     name = "search-friedow-com";

    #     src = ./.;

    #     buildInputs = [
    #       # Node
    #       nodejs

    #       # Tauri
    #       webkitgtk
    #       pkg-config
    #       dbus
    #       openssl

    #       # Rust
    #       rustc
    #       cargo
    #       rustfmt
    #       rustup
    #       rustPackages.clippy
    #     ];

    #     RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
        
    #     shellHook = ''
    #       export PATH="$PWD/node_modules/.bin/:$PATH"
    #     '';

    #     buildPhase = ''
    #       npm run tauri build
    #     '';

    #     installPhase = ''
    #       mkdir -p $out/bin
    #       cp ./src-tauri/target/release/search-friedow-com $out/bin
    #     '';
    #   };

    # };

    defaultPackage.x86_64-linux = self.packages.x86_64-linux.search;

  };
}
