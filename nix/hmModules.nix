inputs:
let
  system = "x86_64-linux";
in
{
  flake.hmModules."${system}".default = import ./home-manager-module-legacy.nix {
    centerpiece = inputs.self.outputs.packages.${system}.default;
    inherit (inputs.self.outputs.packages.${system}) index-git-repositories;
  };
  flake.hmModules.default = import ./home-manager-module.nix inputs.self;
}
