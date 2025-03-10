#!/usr/bin/env bash

BIN_DIR="target/armv7a-none-eabi/debug"
ELF_FILE="$BIN_DIR/rust-bootloader"
BIN_FILE="$BIN_DIR/rust-bootloader.bin"

cargo build --target armv7a-none-eabi --features qemu
arm-none-eabi-objcopy -O binary $ELF_FILE $BIN_FILE

qemu-system-arm -M cubieboard -m 512M -nographic -kernel $BIN_FILE
