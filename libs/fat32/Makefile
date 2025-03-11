# TOOLCHAIN
CROSS_COMPILE ?= arm-none-eabi-
CC ?= gcc

# BUILD PATHS
BUILD_DIR ?= build
SRC_DIR = c_src

AR = ar
RANLIB = ranlib
MKDIR = mkdir -p

# CFLAGS
CFLAGS = -Wall -Wextra -Werror -Wpedantic -g -O2
ifeq ($(CROSS_COMPILE), arm-none-eabi-)
	CFLAGS += -nostdlib -nostartfiles -ffreestanding -fno-builtin -mcpu=cortex-a8 -mfloat-abi=soft -marm
endif

.PHONY: all clean
all: $(BUILD_DIR)/libfat32.a

$(BUILD_DIR)/libfat32.a: $(BUILD_DIR)/fat32.o | $(BUILD_DIR)
	$(CROSS_COMPILE)$(AR) rcs $@ $^
	$(CROSS_COMPILE)$(RANLIB) $@

$(BUILD_DIR)/fat32.o: $(SRC_DIR)/fat32.c | $(BUILD_DIR)
	$(CROSS_COMPILE)$(CC) $(CFLAGS) -c $< -o $@

$(BUILD_DIR):
	$(MKDIR) $@

clean:
	rm -rf $(BUILD_DIR)
