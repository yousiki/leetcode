{
  description = "Leetcode solutions in Rust";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    devshell = {
      url = "github:numtide/devshell";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    leetcode-cli.url = "github:yousiki/leetcode-cli";
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
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
        "x86_64-darwin"
        "aarch64-darwin"
      ];
      perSystem =
        { pkgs, system, ... }:
        {
          _module.args.pkgs = import inputs.nixpkgs {
            inherit system;
            overlays = [
              inputs.leetcode-cli.overlay.${system}
            ];
          };

          treefmt.config = {
            projectRootFile = "flake.nix";
            programs = {
              # Nix formatter
              nixfmt.enable = true;
              # Rust formatter
              rustfmt.enable = true;
              # TOML formatter
              taplo.enable = true;
            };
          };

          devshells.default = {
            name = "leetcode-rs";
            packages = with pkgs; [
              # LeetCode CLI
              leetcode-cli
              # Rust tools
              cargo
              clippy
              rust-analyzer
              rustfmt
              # Nix tools
              nil
              nixd
            ];
            devshell.startup = {
              # Alias for leetcode-cli
              alias.text = ''
                alias lc=leetcode
                alias fmt=treefmt
              '';
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
}
