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

  mov bx, 2
  mov cl, 0

  input:
    mov ah, 0
    int 0x16

    cmp al, 0x08
    je bs

    cmp al, 0x0D
    jne ps

  mov [buffer], byte 0x0a
  mov [buffer + 1], byte 0x0d

  mov ah, 0x0e
  mov bx, buffer

  jmp loopPrint

  jmp shell

ps:
  mov ah, 0x0e
  int 0x10
  inc cl

  mov [buffer + bx], al
  inc bx

  jmp input
  ret
ps1:
  mov ah, 0x0e
  mov al, 0x08
  int 0x10
  mov al, 0x00
  mov [buffer + bx], al
  int 0x10
  mov al, 0x08
  int 0x10

  dec bx
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

loopPrint:
  mov al, [bx]
  cmp al, 0
  je clearbuffer

  int 0x10
  inc bx

  jmp loopPrint
clearbuffer:
  mov al, 0
  mov [buffer + bx], al
  mov bx, 0
  nullchar:
    inc bx
    cmp bx, 258
    je endPrint
    mov al, 0
    mov [buffer + bx], al

    jmp nullchar
endPrint:
  jmp shell
  ret

cls:
  pusha
  mov ah, 0x00
  mov al, 0x03
  int 0x10
  popa
  ret

buffer times 258 db 0

times 510-($-$$) db 0
dw 0xaa55
