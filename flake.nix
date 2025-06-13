{
  description = "Leetcode solutions in Rust";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    devshell = {
      url = "github:numtide/devshell";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } (top: {
      imports = [
        inputs.devshell.flakeModule
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
        { pkgs, ... }:
        {
          devshells.default = {
            name = "leetcode-rs";
            packages = with pkgs; [
              cargo
              clippy
              leetcode-cli
              rust-analyzer
              rustfmt
            ];
            devshell.startup = {
              alias.text = ''
                alias lc=leetcode
              '';
            };
          };
        };
    });
}
