ENTRY(_start)

SECTIONS {
    . = 1M;

    /* ensure the multiboot header is at the beginning */
    .boot BLOCK(4K): ALIGN(4K)
    {
        *(.multiboot)
    }

    .text :
    {
        *(.text)
    }
}
