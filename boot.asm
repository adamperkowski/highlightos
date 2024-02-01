; Copyright (C) 2024  Adam Perkowski

call cls

[org 0x7c00]
  xor ax, ax
  mov ds, ax
  mov es, ax
  mov si, msg
  call print
  jmp $

print:
  lodsb
  or al, al
  jz end
  mov ah, 0x0e
  int 0x10
  jmp print
end:
  ret

msg:
  db 13, 10, "HighlightOS", 13, 10, 10, " Copyright (C) 2024", 13, 10, " Adam Perkowski", 0

cls:
  pusha
  mov ah, 0x00
  mov al, 0x03
  int 0x10
  popa
  ret

times 510-($-$$) db 0
dw 0xaa55