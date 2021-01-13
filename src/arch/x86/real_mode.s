[bits 16]


; A collection of real mode utility function
real_mode:

; Print a string to screen with BIOS's interrupt 0x10, function code 0x0e
; IN: si - the address of the string
; CLOBBER: ax - used for saving the character to print temporarily
.print:
    cld
    mov    ah, 0x0e     ; Function code for teletype output
  .print_loop:
    lodsb
    test   al, al       ; Test if we hit the null character
    jz     .print_done
    int    0x10         ; Call interrupt to print
    jmp    .print_loop
  .print_done:
    ret

; Print a string with new line to screen
; IN: si - the address of the string
; CLOBBER: ax - used for saving the character to print temporarily
.println:
    call   .print
    mov    ah, 0x0e     ; Function code for teletype output
    mov    al, 13       ; \r
    int    0x10         ; Call interrupt to print
    mov    al, 10       ; \n
    int    0x10         ; Call interrupt to print
    ret

; Clear the screen
.clear:
    mov    ah, 0x07    ; Scroll down window
    mov    al, 0x00    ; Clear entire window
    mov    bh, 0x07    ; White on black
    mov    cx, 0x0000  ; row = 0, col = 0
    mov    dx, 0x184f  ; row = 24 (0x18), col = 79 (0x4f)
    int    0x10
    ret

; Move cursor back up to the top left of the screen
.reset_cursor:
    mov     dh, 0x0
    mov     dl, 0x0
    mov     bh, 0x00
    mov     ah, 0x02
    int     0x10
    ret

; read_sectors(u16 num_of_sectors,
;              u32 buffer_addr
;              u32 lba_low, u32 lba_high)
; @notice: buffer_addr is assumed to be within the current segment
.read_sectors:
    push    bp
    mov     bp, sp         ; New stack frame

    ; Using the stack to create the DAP
    ; Upper 32 bits of lba
    mov     eax, [bp + 0xE]
    push    eax

    ; Lower 32 bits of lba
    mov     eax, [bp + 0xA]
    push    eax

    ; Buffer addr
    mov     eax, [bp + 0x6]
    push    eax

    ; Number of sectors
    mov     ax, [bp + 0x4]
    push    ax

    ; Always 1 byte of 0
    xor     ah, ah

    ; Size of pack, assume to be 16 bytes
    mov     al, 0x10
    push    ax

    mov     si, sp
    mov     ah, 0x42
    mov     dl, 0x80         ; "Drive number" - typically 0x80 for C drive
    int     0x13

    add     sp, 0x10
    pop     bp

    ret







