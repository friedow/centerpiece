{
  description = "A very basic flake";

  outputs = { self, nixpkgs }: {

    packages.x86_64-linux.search = with import nixpkgs { system = "x86_64-linux"; }; stdenv.mkDerivation {
      name = "node";
      buildInputs = [
        # Node
        gcc
        gnumake
        python3
        nodejs
        yarn

        # Tauri
        webkitgtk
        pkg-config
        dbus
        openssl

        # Rust
        rustc
        cargo
        rustfmt
        rustup
        rustPackages.clippy 
      ];

      RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
      
      shellHook = ''
        export PATH="$PWD/node_modules/.bin/:$PATH"
      '';
    };

    defaultPackage.x86_64-linux = self.packages.x86_64-linux.search;

  };
}
