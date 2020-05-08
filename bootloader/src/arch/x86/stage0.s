[bits 16]

section .stage0_text
global _boot_entry
_boot_entry:
    ; Disable interrupts and clear direction flag
    cli
    cld

    ; Enable the A20 line
    ; Using the fast way, not aiming for compatibility here
    in    al, 0x92
    or    al, 2
    out   0x92, al

    ; load GDT
    lgdt [gdt.pointer]

    ; Enter protected mode
    mov   eax, cr0
    or    eax, (1 << 0)
    mov   cr0, eax

    ; Clear DS since far jump is relative to DS
    ; xor   ax, ax
    ; mov   ds, ax

    ; Jump with segment to serialize CPU
    jmp   0x0008:_pm_entry

[bits 32]

_pm_entry:
    ; Set all the segment register to the data segment
    mov   ax, 0x0010
    mov   dx, ax
    mov   es, ax
    mov   fs, ax
    mov   gs, ax
    mov   ss, ax


    ; Now we need a rudimentary stack for us to jump into 32bit Rust code
    ; and setup paging and stuff.
    ;
    ; We can just use the space above us (0x7c00)
    ; According to OSDev, we have almost 30KB of free space
    ; before we hit anything related to BIOS.
    mov   esp, 0x7c00
    mov   ebp, esp

extern _bootloader_entry
   call  _bootloader_entry


; ========================= GDT =====================
align 8
gdt:
.null:
    dq 0x0000000000000000 ; 0x00 | Null descriptor
.pm_code:
    dq 0x00cf9a000000ffff ; 0x08 | 32-bit, present, code, base 0
.pm_data:
    dq 0x00cf92000000ffff ; 0x10 | 32-bit, present, data, base 0
.lm_code:
    dq 0x00209a0000000000 ; 0x18 | 64-bit, present, code, base 0
.lm_data:
    dq 0x0000920000000000 ; 0x20 | 64-bit, present, data, base 0
.pointer:
    dw .pointer - gdt - 1
    dd gdt

