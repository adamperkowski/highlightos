; Copyright (C) 2024  Adam Perkowski

call cls

[org 0x7c00]
  xor ax, ax
  mov ds, ax
  mov es, ax
  mov si, msg
  call print

  jmp shell

  jmp $


shell:
  mov ah, 0x0e
  mov al, '>'
  int 0x10

  mov si, nl
  call print

  input:
    mov ah, 0
    int 0x16

    cmp al, 0x08
    je bs

    cmp al, 0x0D
    jne ps

  jmp shell


ps:
  mov ah, 0x0e
  int 0x10
  inc cl
  jmp input
  ret
ps1:
  mov ah, 0x0e
  mov al, 0x08
  int 0x10
  mov al, ' '
  int 0x10
  mov al, 0x08
  int 0x10
  dec cl
  jmp input
  ret

bs:
  cmp cl, 0
  je input
  jne ps1

msg:
  db 13, 10, "HighlightOS", 13, 10, 10, " Copyright (C) 2024", 13, 10, " Adam Perkowski", 0
nl:
  db 13, 10, 10, "hls <", 0

print:
  lodsb
  or al, al
  jz end
  mov ah, 0x0e
  int 0x10
  jmp print
end:
  ret

cls:
  pusha
  mov ah, 0x00
  mov al, 0x03
  int 0x10
  popa
  ret

times 510-($-$$) db 0
dw 0xaa55