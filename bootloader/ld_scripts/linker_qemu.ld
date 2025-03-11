ENTRY(__init)

MEMORY
{
    ROM (rx)  : ORIGIN = 0x40010000, LENGTH = 0x100000  /* 1MB */
    RAM (rwx) : ORIGIN = 0x40110000, LENGTH = 0x4000000 /* 64MB */
}

SECTIONS
{
    . = ORIGIN(ROM);
    .text : {
      KEEP(*(.init))
      *(.text)
    }

    . = ORIGIN(RAM);
    .data : { *(.data) }
    .bss : { *(.bss COMMON) }

    .stack (NOLOAD) : {
        . = ALIGN(16);
        _stack_bottom = .;
        . += 0x1000;
        _stack_top = .;
        __StackStart = .;
    }

    .boot_tables 0x5F610000 (NOLOAD) : {
        _boot_tables_start = .;
        . += 4096 * 4; /* Reserve 16KB for section paging from bootloader */
        _boot_tables_end = .;
    }
    . = ALIGN(4096);
}
