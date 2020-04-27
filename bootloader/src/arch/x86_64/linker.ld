ENTRY(_global_start)

SECTIONS {
    . = 1M;

    /*
     * Boot header has to be at the beginning,
     * so bootloders can find and load the code.
     */
    .boot :
    {
        KEEP(*(.multiboot))
    }

    .text :
    {
        *(.text)
    }

    .bss :
    {
        *(.bss)
    }
}
