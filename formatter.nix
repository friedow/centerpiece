_: {
  projectRootFile = "flake.nix";

  programs = {
    deadnix.enable = true;
    nixfmt.enable = true;
    rustfmt.enable = true;
    statix.enable = true;
    taplo.enable = true;
    yamlfmt.enable = true;
  };
}
