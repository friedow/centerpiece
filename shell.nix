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
        rustPackages.clippy 
    ];
    shellHook = ''
        export PATH="$PWD/node_modules/.bin/:$PATH"
    '';
}