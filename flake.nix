{
  description = "Nix Flake for Rust development";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, rust-overlay }: let
    system = "x86_64-linux";

    pkgs = import nixpkgs {
      inherit system;
      overlays = [ rust-overlay.overlays.default ];
    };

    rust = pkgs.rust-bin.fromRustupToolchainFile ./Toolchain.toml;

    builder = { lib, rustPlatform }: let
      toml = (lib.importTOML ./Cargo.toml).package;
    in rustPlatform.buildRustPackage {
      inherit (toml) version;

      pname = toml.name;
      src = ./.;
      cargoLock.lockFile = ./Cargo.lock;

      meta.mainProgram = "watchers";
    };
  in with pkgs; {
    packages.${system} = {
      watchers = callPackage builder { };
      default = self.packages.${system}.watchers;
    };

    devShells.${system}.default = mkShell {
      packages = [
        rust
        rust-analyzer-unwrapped
        rust-bin.nightly."2024-04-07".rustfmt
      ];
      RUST_SRC_PATH = "${rust}/lib/rustlib/src/rust/library";
    };
  };
}
