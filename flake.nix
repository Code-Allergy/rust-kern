{
  description = "Rust cross-compilation environment for ARMv7a (bare-metal)";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, rust-overlay }: let
    system = "x86_64-linux";
    overlays = [ (import rust-overlay) ];
    pkgs = import nixpkgs { inherit system overlays; };
    rust = pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override {
      extensions = [ "rust-src" "rustfmt" "clippy" ]; # Adds rust-src for cross-compilation
      targets = [ "armv7a-none-eabi" ]; # Pre-installs the Rust target
    });
  in {
    devShells.${system}.default = pkgs.mkShell {
      buildInputs = [
        rust
        pkgs.pkg-config
        pkgs.openssl
        pkgs.gcc-arm-embedded  # ARM bare-metal GCC toolchain
        pkgs.rustup            # Rustup to install the correct target
        pkgs.parted
        pkgs.qemu
        pkgs.llvmPackages.libclang # rustbindgen
      ];

      shellHook = ''
        rustup target add armv7a-none-eabi
        export CARGO_TARGET_ARMV7A_NONE_EABI_LINKER=arm-none-eabi-gcc
        export RUSTFLAGS="-C linker=arm-none-eabi-gcc"
        export LIBCLANG_PATH="${pkgs.llvmPackages.libclang.lib}/lib"
        echo "Rust nightly with ARMv7a-none-eabi cross-compilation setup."
      '';
    };
  };
}
