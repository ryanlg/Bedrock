; Bootstrap using Multiboot 2 

MB_MAGIC       equ 0xE85250D6   ; multiboot 2
MB_ARCH        equ 0            ; i386 Protected mode


section .multiboot
mb_hdr_start:
            dd MB_MAGIC
            dd MB_ARCH
            dd mb_hdr_end - mb_hdr_start
            dd - ( MB_MAGIC + MB_ARCH + (mb_hdr_end - mb_hdr_start) )

            ; required end tag
            dw 0    ; type
            dw 0    ; flags
            dd 8    ; size
mb_hdr_end:



