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
    mov ah, 0x07    ; Scroll down window
    mov al, 0x00    ; Clear entire window
    mov bh, 0x07    ; White on black
    mov cx, 0x0000  ; row = 0, col = 0
    mov dx, 0x184f  ; row = 24 (0x18), col = 79 (0x4f)
    int 0x10
    ret

; Move cursor back up to the top left of the screen
.reset_cursor:
    mov   dh, 0x0
    mov   dl, 0x0
    mov   bh, 0x00
    mov   ah, 0x02
    int   0x10
    ret
