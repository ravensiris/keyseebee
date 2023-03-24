{
  description = "KeySeeBee";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable"; 
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = inputs@{ self, nixpkgs, flake-utils, rust-overlay, ... }:
  flake-utils.lib.eachDefaultSystem (system: let
    overlays = [ (import rust-overlay) ];
    pkgs = import nixpkgs {
      inherit system overlays;
    };
  in rec {
    devShell = pkgs.mkShell {
      CARGO_INSTALL_ROOT = "${toString ./.}/.cargo";

      buildInputs = with pkgs; [
        cargo-binutils
        (rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "llvm-tools-preview" ];
          targets = [ "thumbv6m-none-eabi" ];
        })
        dfu-util
      ];
    };
  });
}
