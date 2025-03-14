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
# export make so it can be checked in the run_qemu.sh script
RUN_QEMU_SCRIPT = MAKE=$(MAKE) ./tools/run_qemu.sh

# Required targets for sd image
BOOTLOADER_ELF = $(RUST_BUILD_DIR)/bootloader
BOOTLOADER_BIN = $(OUTPUT_DIR)/bootloader.bin
BOOTLOADER_MLO = $(OUTPUT_DIR)/MLO

KERNEL_ELF = $(RUST_BUILD_DIR)/kernel
KERNEL_BIN = $(OUTPUT_DIR)/kernel.bin

# Bootloader sources
BOOTLOADER_SRC_DIR = bootloader/src
BOOTLOADER_RS_FILES = $(shell find $(BOOTLOADER_SRC_DIR) -type f -name "*.rs")
BOOTLOADER_C_FILES = $(shell find $(BOOTLOADER_SRC_DIR) -type f -name "*.c")
BOOTLOADER_ASM_FILES = $(shell find $(BOOTLOADER_SRC_DIR) -type f -name "*.S")
BOOTLOADER_SRC_FILES = $(BOOTLOADER_RS_FILES) $(BOOTLOADER_C_FILES) $(BOOTLOADER_ASM_FILES)

# Kernel sources
KERNEL_SRC_DIR = kernel/src
KERNEL_RS_FILES = $(shell find $(KERNEL_SRC_DIR) -type f -name "*.rs")
KERNEL_C_FILES = $(shell find $(KERNEL_SRC_DIR) -type f -name "*.c")
KERNEL_ASM_FILES = $(shell find $(KERNEL_SRC_DIR) -type f -name "*.S")
KERNEL_SRC_FILES = $(KERNEL_RS_FILES) $(KERNEL_C_FILES) $(KERNEL_ASM_FILES)


# Output formatting
BLUE = '\033[0;34m'
NC = '\033[0m'
SPACE = '\040'
PREFIX := "$(BLUE)$(SPACE)$(SPACE)$(SPACE)$(SPACE)Building$(NC)"
RUN_PREFIX := "$(BLUE)$(SPACE)$(SPACE)$(SPACE)$(SPACE)Running$(NC)"

.PHONY: all clean bootloader qemu

all: $(OUT_SDCARD)

################################################################################################
#
# SD card dependency chain
#
################################################################################################
$(OUT_SDCARD): $(BOOTLOADER_MLO) $(KERNEL_BIN)
	@$(MAKE_SDCARD_SCRIPT) $(BOOTLOADER_MLO) $@ $(KERNEL_BIN) | while read line; do \
		echo -e "$(PREFIX) $$line"; \
	done

#
# BOOTLOADER
#
$(BOOTLOADER_MLO): $(BOOTLOADER_BIN) | $(OUTPUT_DIR)
	@$(MAKE_MLO_SCRIPT) $(MLO_DEST_ADDR) $< $@

$(BOOTLOADER_BIN): $(BOOTLOADER_ELF) | $(OUTPUT_DIR)
	@echo -e "$(PREFIX) Creating bootloader flat binary from rust build..."
	@arm-none-eabi-objcopy -O binary $< $@

$(BOOTLOADER_ELF): $(BOOTLOADER_SRC_FILES)
	@echo -e "$(PREFIX) Calling cargo to build bootloader..."
	@CARGO_TARGET_DIR=$(BUILD_DIR) cargo build $(CARGO_FLAGS) -p bootloader \
	    --features "boot_mmc"

#
# KERNEL
#
$(KERNEL_BIN): $(KERNEL_ELF) | $(OUTPUT_DIR)
	@echo -e "$(PREFIX) Creating kernel flat binary from rust build..."
	@arm-none-eabi-objcopy -O binary $< $@

$(KERNEL_ELF): $(KERNEL_SRC_FILES)
	@echo -e "$(PREFIX) Calling cargo to build kernel..."
	@CARGO_TARGET_DIR=$(BUILD_DIR) cargo build $(CARGO_FLAGS) -p kernel

$(OUTPUT_DIR):
	@mkdir -p $@


#
# QEMU boot os
#
qemu:
	@$(MAKE) _qemu PLATFORM=qemu

_qemu: $(OUT_SDCARD) $(BOOTLOADER_BIN)
	@MAKE=$(MAKE) ./tools/run_qemu.sh $(KERNEL_BIN)

qemu_gdb:
	@$(MAKE) _qemu_gdb PLATFORM=qemu

qemu-gdb:
	@$(MAKE) _qemu_gdb PLATFORM=qemu

_qemu_gdb: $(OUT_SDCARD) $(BOOTLOADER_BIN)
	@MAKE=$(MAKE) ./tools/run_qemu.sh $(KERNEL_BIN) --gdb

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
