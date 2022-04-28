{ systemPkgs ? import <nixpkgs> {} }:

let unstable = import (systemPkgs.fetchFromGitHub {
		owner = "NixOS";
		repo  = "nixpkgs";
		rev   = "3fdd780";
		hash  = "sha256:0df9v2snlk9ag7jnmxiv31pzhd0rqx2h3kzpsxpj07xns8k8dghz";
	}) {
		overlays = [
			(self: super: {
				go = super.go.overrideAttrs (old: {
					version = "1.18";
					src = builtins.fetchurl {
						url    = "https://golang.org/dl/go1.18.linux-amd64.tar.gz";
						sha256 = "0kr6h1ddaazibxfkmw7b7jqyqhskvzjyc2c4zr8b3kapizlphlp8";
					};
					doCheck = false;
					patches = [
						# cmd/go/internal/work: concurrent ccompile routines
						(builtins.fetchurl "https://github.com/diamondburned/go/commit/ec3e1c9471f170187b6a7c83ab0364253f895c28.patch")
						# cmd/cgo: concurrent file generation
						(builtins.fetchurl "https://github.com/diamondburned/go/commit/50e04befeca9ae63296a73c8d5d2870b904971b4.patch")
					];
				});
			})
		];
	};

	lib = systemPkgs.lib;

	gtkPkgs =
		if ((systemPkgs.gtk4 or null) != null && lib.versionAtLeast systemPkgs.gtk4.version "4.4.0")
		then systemPkgs
		else unstable;

in gtkPkgs.mkShell {
	buildInputs = with gtkPkgs; [
		glib
		graphene
		gdk-pixbuf
		gnome3.gtk
		gtk4
		vulkan-headers
	];

	nativeBuildInputs = with gtkPkgs; [
		# Build/generation dependencies.
		gobjectIntrospection
		pkgconfig

		unstable.go

		# Development tools.
		# gopls
		# goimports
	];

	NIX_DEBUG_INFO_DIRS = ''${gtkPkgs.gtk4.debug}/lib/debug:${gtkPkgs.glib.debug}/lib/debug'';
	CGO_ENABLED = "1";

	TMP    = "/tmp";
	TMPDIR = "/tmp";
}
