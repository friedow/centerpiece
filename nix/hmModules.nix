self:
let
  system = "x86_64-linux";
in
{
  flake.hmModules."${system}".default = import ./home-manager-module-legacy.nix {
    centerpiece = self.outputs.packages.${system}.default;
    inherit (self.outputs.packages.${system}) index-git-repositories;
  };
  flake.hmModules.default = import ./home-manager-module.nix self;
}
