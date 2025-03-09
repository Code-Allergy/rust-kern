PLATFORM ?= qemu

# Define build directories and output files as functions of PLATFORM
define get_build_dir
$(if $(filter bbb,$(1)),target/deploy,target/qemu)
endef

define get_features
$(if $(filter bbb,$(1)),bbb,qemu)
endef

# Default settings
BUILD_DIR = $(call get_build_dir,$(PLATFORM))
FEATURES = $(call get_features,$(PLATFORM))

MLO_DEST_ADDR = 0x402f0400
OUT_SDIMG = $(BUILD_DIR)/sd.img
OUT_MLO = $(BUILD_DIR)/MLO
OUT_BIN = $(BUILD_DIR)/bootloader.bin
OUT_ELF = target/armv7a-none-eabi/debug/rust-bootloader

.PHONY: all build-rust debug clean build-bbb build-qemu flash qemu qemu-gdb

all: $(OUT_SDIMG)

# For flash, explicitly recalculate paths based on bbb platform
flash:
	$(MAKE) _do_flash DEV=$(DEV) PLATFORM=bbb

_do_flash: $(call get_build_dir,bbb)/sd.img
ifndef DEV
	$(error DEV is not set. Run "make flash DEV=path/to/dev" to flash the image)
endif
	sudo ./tools/flash_img.sh $< $(DEV)

# For qemu, explicitly recalculate paths based on qemu platform
qemu:
	$(MAKE) _do_qemu PLATFORM=qemu

_do_qemu: $(call get_build_dir,qemu)/bootloader.bin
	qemu-system-arm -m 512M -M cubieboard \
	-cpu cortex-a8 \
	-serial mon:stdio -nographic \
	-kernel $< \
	-d guest_errors,unimp,int -D qemu.log

qemu-gdb:
	$(MAKE) _do_qemu_gdb PLATFORM=qemu

_do_qemu_gdb: $(call get_build_dir,qemu)/bootloader.bin
	qemu-system-arm -m 512M -M cubieboard \
	-cpu cortex-a8 \
	-serial mon:stdio -nographic \
	-kernel $< \
	-d guest_errors,unimp,int -S -gdb tcp::1234

# General build rules
$(BUILD_DIR)/sd.img: $(BUILD_DIR)/MLO
	./tools/mksdimage.sh $< $@

$(BUILD_DIR)/MLO: $(BUILD_DIR)/bootloader.bin
	./tools/mk-gpimage $(MLO_DEST_ADDR) $< $@

$(BUILD_DIR)/bootloader.bin: $(OUT_ELF) | $(BUILD_DIR)
	arm-none-eabi-objcopy -O binary $(OUT_ELF) $@

# This rule builds the ELF file with the appropriate features
$(OUT_ELF): src/main.rs src/boot.S
	cargo build --features $(FEATURES)

build-rust: $(OUT_ELF)

build-bbb:
	$(MAKE) build-rust PLATFORM=bbb

build-qemu:
	$(MAKE) build-rust PLATFORM=qemu

debug:
	cargo build

$(BUILD_DIR):
	mkdir -p $@

clean:
	cargo clean
	rm -rf target/deploy target/qemu
