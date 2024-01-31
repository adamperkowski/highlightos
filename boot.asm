; Copyright (C) 2024  Adam Perkowski

call cls

[org 0x7c00]
mov ah, 0x0e
mov bx, text

printString:
    mov al, [bx]
    cmp al, 0
    je end
    int 0x10
    inc bx
    jmp printString
end:

text:
    db "HighlightOS", 13, 10, 10, " Copyright (C) 2024", 13, 10, " Adam Perkowski", 0

jmp $

cls:
  pusha
  mov ah, 0x00
  mov al, 0x03
  int 0x10
  popa
  ret

times 510-($-$$) db 0
dw 0xaa55
