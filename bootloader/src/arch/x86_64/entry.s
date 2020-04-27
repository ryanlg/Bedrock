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
            call _check_multiboot

            mov ebp, stack_top
            mov esp, stack_top

; @mp: Currently this bootloader assumes it has exclusive
; control over all resources, this will not work with
; multiple processors down the line, if we ever get to that point.
; But now this will suffice.
extern _kernel_entry
            call _kernel_entry

; Should not happen, but if happens, halt the cpu
_kernel_return:
            cli
_infi_hlt:
            hlt
            jmp _infi_hlt

; Check if the bootloader successfully loaded the kernel
_check_multiboot:
            cmp eax, MB_MAGIC_CHECK
            jne _fail_multiboot
            ret

_fail_multiboot:
            mov dword [0xb8000], 0x2f4f2f4e    ; Prints green 'NO'
            cli
            hlt
