org 0x7C00
bits 16


%define NLINE 0x0D, 0x0A

start:
    jmp main

puts:
    push si
    push ax

.loop:
    lodsb 
    or al, al
    jz .done    ;jump if next character null

    mov ah, 0x0e
    xor bh, bh
    int 0x10


    jmp .loop
    
.done:
    pop ax
    pop si
    ret

main:
    cli   ;disabling interrupts
    xor ax, ax   ;sets ax to zero (could use mov ax, 0)
    mov ds, ax
    mov es, ax
    mov ss, ax
    mov sp, 0x7C00   ;setup stack pointer

    mov si, msg
    call puts

    hlt


.halt:
    jmp .halt


msg db "Booting Kernel...", NLINE, 0

times 510-($-$$) db 0
dw 0AA55h