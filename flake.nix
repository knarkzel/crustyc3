{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    flake-compat.url = "https://flakehub.com/f/edolstra/flake-compat/1.tar.gz";
    rust = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
    };
  };

  outputs = { nixpkgs, flake-utils, rust, ... }:
    flake-utils.lib.eachDefaultSystem (system: let
      overlays = [
        rust.overlays.default
      ];
      pkgs = import nixpkgs {inherit system overlays;};
    in {
      devShell = pkgs.mkShell {
        buildInputs = with pkgs; [
          cargo-espflash
          (rust-bin.nightly.latest.default.override {
            extensions = ["rust-src"];
            targets = ["riscv32imc-unknown-none-elf"];
          })
        ];
      };

      shellHook = ''
        export ESP_LOGLEVEL="INFO"
      '';
    });
}
