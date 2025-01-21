self:
let
  system = "x86_64-linux";
in
{
  flake.hmModules."${system}".default = import ./home-manager-module-legacy.nix {
    centerpiece = self.packages.${system}.default;
    inherit (self.packages.${system}) index-git-repositories;
  };
  flake.hmModules.default = import ./home-manager-module.nix self;
}
