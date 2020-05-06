ENTRY(_boot_entry)

SECTIONS {
    . = 0x7c00;

    .text :
    {
        __start = .;

        *(.text)

        __text_end = .;
        __text_size = __text_end - __start;

        /* Magic number
         * We could have put this in the actual stage0.s, but TBH this
         * is a better place since we don't have to deal with
         * janky assembly address math
         */
        . = . + (0x200 - 0x2 - __text_size);
        SHORT(0xAA55);

        ASSERT(__text_size < 512, "Text Segment too big for MBR");
    }

    /**
     * GDT info is in rodata
     */
    . = ALIGN(16);
    .rodata :        { *(.rodata*) }

}
