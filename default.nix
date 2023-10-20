{rustPlatform}:
rustPlatform.buildRustPackage {
  name = "rust-cross-build";
  src = ./.;
  cargoLock.lockFile = ./Cargo.lock;
}
