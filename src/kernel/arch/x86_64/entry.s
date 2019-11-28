; Prepare a stack for to kernel code execution

; %eax must contain this value after loaded by multiboot2 bootloader
MB_MAGIC_CHECK     equ 0x36d76289

; Reserve memory for the stack
            section .bss
            align 16
stack_bottom:            ; Grows down
            resb 16384   ; Reserve 16KB
stack_top:

            section .text
            bits 32
            global _global_start
_global_start:
            mov ebp, stack_top
            mov esp, stack_top

            call _check_multiboot

            mov dword [0xb8000], 0x2f4b2f4f    ; Prints green 'OK'
            hlt

; Check if the bootloader successfully loaded the kernel
_check_multiboot:
            cmp eax, MB_MAGIC_CHECK
            jne _fail_multiboot
            ret

_fail_multiboot:
            mov dword [0xb8000], 0x2f4f2f4e    ; Prints green 'NO'
            cli
            hlt
