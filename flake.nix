{
  description = "Dev env for vi-rs";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, ... }@inputs: flake-utils.lib.eachDefaultSystem (system:
    let
      rustComponents = [
        "cargo"
        "clippy"
        "rust-src"
        "rustc"
        "rustfmt"
      ];
      pkgs = nixpkgs.legacyPackages.${system}.extend inputs.fenix.overlays.default;
      commonBuildInputs = with pkgs; [
        rust-analyzer
      ];
    in
    {
      devShells.default = pkgs.mkShell {
        buildInputs = with pkgs; [
          (fenix.stable.withComponents rustComponents)
        ] ++ commonBuildInputs;
      };
    }
  );
}
