

BUILD_DIR = target/deploy
MLO_DEST_ADDR = 0x402f0400

OUT_SDIMG = $(BUILD_DIR)/sd.img
OUT_MLO = $(BUILD_DIR)/MLO
OUT_BIN = $(BUILD_DIR)/bootloader.bin
OUT_ELF = target/armv7a-none-eabi/release/rust-bootloader

.PHONY: all build-rust debug clean
all: $(OUT_SDIMG)

flash: $(OUT_SDIMG)
ifndef DEV
		$(error DEV is not set. Run "make flash DEV=path/to/dev" to flash the image)
endif
		sudo ./tools/flash_img.sh $(OUT_SDIMG) $(DEV)


$(OUT_SDIMG): $(OUT_MLO)
	./tools/mksdimage.sh $(BUILD_DIR)/MLO $(OUT_SDIMG)

$(OUT_MLO): $(OUT_BIN)
	./tools/mk-gpimage $(MLO_DEST_ADDR) $< $@

$(OUT_BIN): $(OUT_ELF) | $(BUILD_DIR)
	arm-none-eabi-objcopy -O binary $< $@

$(OUT_ELF): build-rust



build-rust: src/main.rs src/boot.S
	cargo build --release

debug:
	cargo build


$(BUILD_DIR):
	mkdir -p $@

clean:
	cargo clean
