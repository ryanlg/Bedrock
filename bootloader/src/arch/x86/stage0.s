[bits 16]

; Linker will put this section in .text
; Separating this out so we have more control over where things are
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

    ; Now we need a rudimentary stack for us to call some basic utilities
    ; and to jump into 32bit Rust code to setup paging and stuff.
    ;
    ; We can just use the space above us (0x7c00)
    ; According to OSDev, we have almost 30KB of free space
    ; before we hit anything related to BIOS.
    mov   esp, 0x7c00
    mov   ebp, esp

    ; Clear the window
    call  real_mode.clear
    call  real_mode.reset_cursor

    mov   si, real_mode_strings.live
    call  real_mode.println

    mov   si, real_mode_strings.switch
    call  real_mode.println

    ; Enter protected mode
    mov   eax, cr0
    or    eax, (1 << 0)
    mov   cr0, eax

    ; Jump with segment to serialize CPU
    jmp   0x0008:_pm_entry

%include "src/arch/x86/real_mode.s"

[bits 32]

_pm_entry:
    ; Set all the segment register to the data segment
    mov   ax, 0x0010
    mov   dx, ax
    mov   es, ax
    mov   fs, ax
    mov   gs, ax
    mov   ss, ax

extern _bootloader_entry
   call  _bootloader_entry


; ========================= GDT =====================
align 8
gdt:
    dq 0x0000000000000000 ; 0x00 | Null descriptor
    dq 0x00cf9a000000ffff ; 0x08 | 32-bit, present, code, base 0
    dq 0x00cf92000000ffff ; 0x10 | 32-bit, present, data, base 0
    dq 0x00209a0000000000 ; 0x18 | 64-bit, present, code, base 0
    dq 0x0000920000000000 ; 0x20 | 64-bit, present, data, base 0
.pointer:
    dw .pointer - gdt - 1
    dd gdt

; ====================== Strings =====================
real_mode_strings:
.live:        db  'We are live and in real mode', 0
.switch:      db  'We are switching to protected mode', 0

; The MBR magic number is added in the linker script
