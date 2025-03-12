{
  description = "Flake for my osdev project";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
    }:
    let
      supportedSystems = [
        "x86_64-linux"
        "aarch64-linux"
      ];
      forAllSystems = nixpkgs.lib.genAttrs supportedSystems;
    in
    {
      packages = forAllSystems (
        system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ (import rust-overlay) ];
          };
        in
        {
          default = self.packages.${system}.qemu-run;
          qemu-run = pkgs.writeShellScriptBin "run-qemu" ''...'';
        }
      );
      devShells = forAllSystems (
        system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ (import rust-overlay) ];
          };

          baseInputs = [
            (pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml)
            pkgs.gcc-arm-embedded
            pkgs.llvmPackages.libclang
          ];
          commonEnv = {
            RUSTFLAGS = "";
            LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
          };
        in
        {
          minimal = pkgs.mkShell {
            buildInputs = baseInputs;
            inherit (commonEnv) RUSTFLAGS LIBCLANG_PATH;
          };
          dev = pkgs.mkShell {
            buildInputs = baseInputs ++ [
              pkgs.qemu
              pkgs.minicom
              pkgs.mtools
              pkgs.parted
              pkgs.fatcat
            ];
            inherit (commonEnv) RUSTFLAGS LIBCLANG_PATH;
          };
          debug = pkgs.mkShell {
            buildInputs = baseInputs ++ [
              pkgs.gdb
            ];
            inherit (commonEnv) RUSTFLAGS LIBCLANG_PATH;
          };
          full = pkgs.mkShell {
            inputsFrom = [
              self.devShells.${system}.minimal
              self.devShells.${system}.dev
              self.devShells.${system}.debug
            ];
            inherit (commonEnv) RUSTFLAGS LIBCLANG_PATH;
          };
          default = self.devShells.${system}.full;
        }
      );

      checks = forAllSystems (
        system:
        let
          pkgs = import nixpkgs { inherit system; };
        in
        {
          fmt =
            pkgs.runCommand "check-fmt"
              {
                buildInputs = [ pkgs.rustfmt ];
              }
              ''
                cargo fmt -- --check
                touch $out
              '';
        }
      );
    };
}
