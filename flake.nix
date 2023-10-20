{
  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
  }: let
    system = "x86_64-linux";
    overlays = [(import rust-overlay)];
    pkgs = import nixpkgs {
      inherit overlays system;
      crossSystem = {
        config = "armv7a-unknown-linux-gnueabihf";
        rustc.config = "armv7-unknown-linux-gnueabihf";
      };
    };
  in {
    packages.${system} = {
      default = self.outputs.packages.${system}.armv7a-linux-buildhf;
      armv7a-linux-buildhf = pkgs.callPackage ./. {};
    };
  };
}
