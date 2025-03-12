#!/usr/bin/env bash
set -euo pipefail

# Default paths
# Where files are built for qemu
DEFAULT_DEPLOY_DIR='deploy/qemu'
BOOTBIN_FILE="$DEFAULT_DEPLOY_DIR/bootloader.bin"
DEFAULT_SDCARD_PATH="$DEFAULT_DEPLOY_DIR/sdcard.img"

# Temp - this should be dynamic (for rust at least)
DEFAULT_ELFBIN_PATH="target/qemu/armv7a-none-eabi/debug/bootloader"

# check if args > 3
if [ "$#" -lt 1 ]; then
    echo "Usage: $0 [bootloader/kernel] < --gcc >"
    exit 1
fi


RUN_ARG_BIN=$(basename $1)
echo "Building for: $RUN_ARG_BIN"

if [ "$RUN_ARG_BIN" = "bootloader" ]; then
    BOOTELF_FILE=$1
    SDCARD_IMG=""
else
    BOOTELF_FILE=$DEFAULT_ELFBIN_PATH
    SDCARD_IMG=$DEFAULT_SDCARD_PATH
    make $SDCARD_IMG PLATFORM=qemu
fi


GDB_PORT="1234"

# if we have an elf file passed in and bootbin file is not found, then we need to build it
if [ -f $BOOTELF_FILE ]; then
    echo "Bootloader file found at $BOOTELF_FILE"
    # Check if the bootbin file is found
    if [ ! -f "$BOOTBIN_FILE" ]; then
        echo "Bootloader file not found at $BOOTBIN_FILE"
        echo "Building bootloader from $BOOTELF_FILE"
        mkdir -p $(dirname $BOOTBIN_FILE)
        arm-none-eabi-objcopy -O binary $BOOTELF_FILE $BOOTBIN_FILE
    fi
fi

# Exit if bootbin file is not found
if [ ! -f "$BOOTBIN_FILE" ]; then
    echo "Bootloader file not found at $BOOTBIN_FILE"
    exit 1
fi

# If there's an sd card argument, check if the image size is a power of 2
echo "SD card image: $SDCARD_IMG"
if [ -n "$SDCARD_IMG" ]; then
    # Get the disk image size using qemu-img info
    IMG_SIZE=$(qemu-img info $SDCARD_IMG | grep "virtual size" | awk '{print $3}' | sed 's/[^0-9]//g')
    nearest_power_of_two() {
        local number=$1

        # Subtract 1 and then round up to the nearest power of 2 using bit manipulation
        local rounded_up=$(( (number - 1) | (number - 1) >> 1 ))
        rounded_up=$(( rounded_up | rounded_up >> 2 ))
        rounded_up=$(( rounded_up | rounded_up >> 4 ))
        rounded_up=$(( rounded_up | rounded_up >> 8 ))
        rounded_up=$(( rounded_up | rounded_up >> 16 ))

        # Add 1 to get the next power of 2
        local result=$(( rounded_up + 1 ))

        echo $result
    }
    POWER_OF_TWO=$(nearest_power_of_two $IMG_SIZE)
    qemu-img resize -f raw $SDCARD_IMG "${POWER_OF_TWO}M"
else
    echo "Running without SD card image"
fi




SYSTEM_ARGS="-m 512M -M cubieboard -cpu cortex-a8"
OUTPUT_ARGS="-serial mon:stdio -nographic"
LOG_ARGS="-d guest_errors,unimp,int -D qemu.log"


# Log a message if no sd card image is found
if [ ! -f "$SDCARD_IMG" ]; then
    SDCARD_FLAGS=""
else
    SDCARD_FLAGS="-drive if=sd,format=raw,file=$SDCARD_IMG"
fi

BOOTLOADER_FLAGS="-kernel $BOOTBIN_FILE"

# Check if second argument is "--gdb"
if [[ -n "${2:-}" && "$2" == "--gdb" ]]; then
    GDB_ARGS="-S -gdb tcp::$GDB_PORT"
    echo "Running with GDB server on port $GDB_PORT"
    echo "Connect with: gdb -ex 'target remote localhost:$GDB_PORT'"
    shift  # Shift to move the argument
else
    GDB_ARGS=""  # Default to an empty string if not using GDB
fi

QEMU_CMD="qemu-system-arm $SYSTEM_ARGS $OUTPUT_ARGS $LOG_ARGS $SDCARD_FLAGS $BOOTLOADER_FLAGS $GDB_ARGS"

echo $QEMU_CMD
$QEMU_CMD
