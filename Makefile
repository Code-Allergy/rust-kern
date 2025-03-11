# default cargo build mode
BUILD_MODE ?= debug
ifeq ($(BUILD_MODE), release)
	CARGO_FLAGS += --release
endif

PLATFORM ?= qemu
ifeq ($(PLATFORM), qemu)
	CARGO_FLAGS += --no-default-features --features qemu
	MLO_DEST_ADDR = 0x00000000 # A10 allwinner doesn't use MLO
else ifeq ($(PLATFORM), bbb)
	CARGO_FLAGS += --no-default-features --features bbb
	MLO_DEST_ADDR = 0x402f0400
else
	$(error Unknown platform $(PLATFORM))
endif

BUILD_DIR ?= target/$(PLATFORM)
OUTPUT_BASE_DIR = deploy
OUTPUT_DIR = $(OUTPUT_BASE_DIR)/$(PLATFORM)

RUST_TRIPLE = armv7a-none-eabi
RUST_BUILD_DIR = $(BUILD_DIR)/$(RUST_TRIPLE)/$(BUILD_MODE)


OUT_SDCARD = $(OUTPUT_DIR)/sdcard.img

# Scripts
MAKE_SDCARD_SCRIPT = ./tools/mksdimage.sh
MAKE_MLO_SCRIPT = ./tools/mk-gpimage
FLASH_BBB_SCRIPT = sudo ./tools/flash_bbb.sh

# Required targets for sd image
BOOTLOADER_ELF = $(RUST_BUILD_DIR)/bootloader
BOOTLOADER_BIN = $(OUTPUT_DIR)/bootloader.bin
BOOTLOADER_MLO = $(OUTPUT_DIR)/MLO

KERNEL_ELF = $(RUST_BUILD_DIR)/kernel
KERNEL_BIN = $(OUTPUT_DIR)/kernel.bin

# Bootloader sources
BOOTLOADER_SRC_DIR = bootloader
BOOTLOADER_RS_FILES = $(shell find $(BOOTLOADER_SRC_DIR) -type f -name "*.rs")
BOOTLOADER_C_FILES = $(shell find $(BOOTLOADER_SRC_DIR) -type f -name "*.c")
BOOTLOADER_ASM_FILES = $(shell find $(BOOTLOADER_SRC_DIR) -type f -name "*.S")
BOOTLOADER_SRC_FILES = $(BOOTLOADER_RS_FILES) $(BOOTLOADER_C_FILES) $(BOOTLOADER_ASM_FILES)

.PHONY: all clean bootloader qemu

all: $(OUT_SDCARD)

################################################################################################
#
# SD card dependency chain
#
################################################################################################
$(OUT_SDCARD): $(BOOTLOADER_MLO) $(KERNEL_BIN)
	@$(MAKE_SDCARD_SCRIPT) $(BOOTLOADER_MLO) $@ $(KERNEL_BIN)

#
# BOOTLOADER
#
$(BOOTLOADER_MLO): $(BOOTLOADER_BIN) | $(OUTPUT_DIR)
	@$(MAKE_MLO_SCRIPT) $(MLO_DEST_ADDR) $< $@

$(BOOTLOADER_BIN): $(BOOTLOADER_ELF) | $(OUTPUT_DIR)
	@echo "Creating bootloader flat binary from rust build..."
	@arm-none-eabi-objcopy -O binary $< $@

$(BOOTLOADER_ELF): $(BOOTLOADER_SRC_FILES)
	CARGO_TARGET_DIR=$(BUILD_DIR) cargo build $(CARGO_FLAGS) -p bootloader

#
# KERNEL
#
$(KERNEL_BIN):
	@echo "Hello, world!" > $@

$(OUTPUT_DIR):
	@mkdir -p $@


#
# QEMU boot os
#
qemu:
	@$(MAKE) _qemu PLATFORM=qemu

_qemu: $(OUT_SDCARD) $(BOOTLOADER_BIN)
	@qemu-img resize $(OUT_SDCARD) 128M
	qemu-system-arm -m 512M -M cubieboard \
	-cpu cortex-a8 \
	-serial mon:stdio -nographic \
	-drive if=sd,format=raw,file=$(OUT_SDCARD) \
	-d guest_errors,unimp,int -D qemu.log \
	-kernel $(BOOTLOADER_BIN)

flash:
	@$(MAKE) _flash PLATFORM=bbb

_flash: $(OUT_SDCARD)
ifndef DEV
	$(error DEV is not set)
endif
	@$(FLASH_BBB_SCRIPT) $(OUT_SDCARD) $(DEV)


clean:
	cargo clean
	rm -rf $(OUTPUT_BASE_DIR)
