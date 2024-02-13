{ index-git-repositories, centerpiece }:
{ lib, pkgs, config, ... }:
let
  cfg = config.programs.centerpiece;
  git-index-name = "index-git-repositories";
in {
  options.programs.centerpiece = {
    enable = lib.mkEnableOption (lib.mdDoc "Centerpiece");

    config.plugin.git_repositories = {
      editor_command = lib.mkOption {
        default = [
          [ "alacritty" "--command" "nvim" "$GIT_DIRECTORY" ]
          [ "alacritty" "--working-directory" "$GIT_DIRECTORY" ]
        ];
        type = lib.types.listOf lib.types.listOf lib.types.str;
        description = lib.mdDoc
          "The commands to launch when an entry is selected. Use the $GIT_DIRECTORY variable to pass in the selected directory.";
        example = [
          [ "code" "--new-window" "$GIT_DIRECTORY" ]
          [ "alacritty" "--command" "lazygit" "--path" "$GIT_DIRECTORY" ]
          [ "alacritty" "--working-directory" "$GIT_DIRECTORY" ]
        ];
      };
    };

    services.index-git-repositories = {
      enable = lib.mkEnableOption (lib.mdDoc "enable timer");
      interval = lib.mkOption {
        default = "5min";
        type = lib.types.str;
        example = "hourly";
        description = lib.mdDoc ''
          Frequency of index creation.

          The format is described in
          {manpage}`systemd.time(7)`.
        '';
      };
    };
  };

  config = lib.mkMerge [
    (lib.mkIf cfg.enable { home.packages = [ centerpiece ]; })

    (lib.mkIf cfg.config {
      home.file.".config/centerpiece/config.yml" = builtins.toYAML cfg.config;
    })

    (lib.mkIf cfg.services.index-git-repositories.enable {
      systemd.user = {
        services = {
          index-git-repositories-service = {
            Unit = {
              Description = "Centerpiece - your trusty omnibox search";
              Documentation = "https://github.com/friedow/centerpiece";
            };

            Service = {
              ExecStart = "${pkgs.writeShellScript
                "${git-index-name}-service-ExecStart" ''
                  exec ${lib.getExe index-git-repositories}
                ''}";
              Type = "oneshot";
            };
          };
        };
        timers = {
          index-git-repositories-timer = {
            Unit = { Description = "Activate the git repository indexer"; };
            Install = { WantedBy = [ "timers.target" ]; };
            Timer = {
              OnUnitActiveSec = cfg.services.index-git-repositories.interval;
              OnBootSec = "0min";
              Persistent = true;
              Unit = "${git-index-name}-service.service";
            };
          };
        };
      };
    })
  ];
}
