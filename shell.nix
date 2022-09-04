with import <nixpkgs> {};

stdenv.mkDerivation {
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
}
