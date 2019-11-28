ENTRY(_global_start)

SECTIONS {
    . = 1M;

    /*
     * Boot header has to be at the beginning,
     * so bootloders can find and load the code.
     */
    .boot BLOCK(4K): ALIGN(4K)
    {
        *(.multiboot)
    }

    .text :
    {
        *(.text)
    }

    .bss BLOCK(4K): ALIGN(4K)
    {
        *(.bss)
    }
}
