{
  description = "Flake for my osdev project";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
  }: let
    system = "x86_64-linux";
    pkgs = import nixpkgs {
      inherit system;
      overlays = [(import rust-overlay)];
    };

    commonEnv = {
      RUSTFLAGS = "--build-id=none";
      LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
    };

    baseInputs = [
      (pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml)
      pkgs.gcc-arm-embedded
      pkgs.llvmPackages.libclang
    ];
  in {
    packages.${system} = {
    default = self.packages.${system}.qemu-run;
    qemu-run = pkgs.writeShellScriptBin "run-qemu" ''...'';
    };

    devShells.${system} = {
      minimal = pkgs.mkShell {
        buildInputs = baseInputs;
        inherit (commonEnv) RUSTFLAGS LIBCLANG_PATH;
      };
      dev = pkgs.mkShell {
        buildInputs =
          baseInputs
          ++ [
            pkgs.qemu
            pkgs.minicom
            pkgs.mtools
            pkgs.parted
            pkgs.fatcat
          ];
        inherit (commonEnv) RUSTFLAGS LIBCLANG_PATH;
      };
      debug = pkgs.mkShell {
        buildInputs =
          baseInputs
          ++ [
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
      };
      default = self.devShells.${system}.full;
    };

    checks.${system} = {
      fmt =
        pkgs.runCommand "check-fmt" {
          buildInputs = [pkgs.rustfmt];
        } ''
          cargo fmt -- --check
          touch $out
        '';
    };
  };
}
