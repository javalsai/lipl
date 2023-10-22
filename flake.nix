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
    specialArgs = import ./config.nix;
    system = "x86_64-linux";
    overlays = [(import rust-overlay)];
    pkgs = import nixpkgs {
      inherit overlays system;
      crossSystem = {
        config = specialArgs.targets.nix;
        rustc.self.config = specialArgs.targets.rust;
      };
    };
  in {
    packages.${system} = {
      default = self.outputs.packages.${system}.${specialArgs.targets.nix};
      ${specialArgs.targets.nix} = pkgs.callPackage ./. {};
    };
  };
}
