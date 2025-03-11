ENTRY(_start)

SECTIONS {
    . = 0xA0000000;
    .text : {
        *(.text._start)  /* Place `_start` first */
        *(.text)
    }
    .data : {
        *(.data)
    }
    .bss : {
        *(.bss)
    }
}
