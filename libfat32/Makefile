CC = arm-none-eabi-gcc
CFLAGS = -Wall -Wextra -Werror -Wpedantic -g -O2 -nostdlib -nostartfiles -ffreestanding -fno-builtin -mcpu=cortex-a8 -mfloat-abi=soft -marm

.PHONY: all clean
all: libfat32.a

libfat32.a: fat32.o
	arm-none-eabi-ar rcs $@ $^
	arm-none-eabi-ranlib $@

fat32.o: fat32.c
	$(CC) $(CFLAGS) -c $< -o $@

clean:
	rm -f *.o *.a
