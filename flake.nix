{
  description = "Leetcode solutions in Rust";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    devshell = {
      url = "github:numtide/devshell";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    leetcode-cli.url = "github:yousiki/leetcode-cli";
  };

  outputs =
    inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } (top: {
      imports = [
        inputs.devshell.flakeModule
        inputs.treefmt-nix.flakeModule
      ];
      flake = {
        # Put your original flake attributes here.
      };
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];
      perSystem =
        { pkgs, system, ... }:
        {
          _module.args.pkgs = import inputs.nixpkgs {
            inherit system;
            overlays = [
              inputs.leetcode-cli.overlays.default
            ];
          };

          treefmt.config = {
            projectRootFile = "flake.nix";
            programs = {
              nixfmt.enable = true;
              rustfmt.enable = true;
              taplo.enable = true;
            };
          };

          devshells.default = {
            name = "leetcode-rs";
            packages = with pkgs; [
              # LeetCode CLI
              leetcode-cli
              # Rust tools
              rustc
              cargo
              clippy
              rust-analyzer
              rustfmt
              # Nix tools
              nil
              nixd
            ];
            devshell.startup = {
              # Set the environment variables
              env.text = ''
                if [ -f ./.env ]; then
                  source ./.env
                fi
              '';
            };
          };
        };
    });

  nixConfig = {
    extra-substituters = [
      "https://nichijou.cachix.org"
    ];
    extra-trusted-public-keys = [
      "nichijou.cachix.org-1:rbaTU9nLgVW9BK/HSV41vsag6A7/A/caBpcX+cR/6Ps="
    ];
  };
}
