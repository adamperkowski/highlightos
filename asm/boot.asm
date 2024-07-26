; Copyleft 🄯 2024  Adam Perkowski
[org 0x7c00]
section .text
global _start

_start:
  call cls
  xor ax, ax
  mov ds, ax
  mov es, ax
  mov si, boot_msg
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

.input:
  mov ah, 0
  int 0x16

  cmp al, 0x08
  je .buffer_bounds

  cmp al, 0x0D
  jne .print_input

  mov [buffer], byte 0x0a
  mov [buffer + 1], byte 0x0d

  mov ah, 0x0e
  mov bx, buffer

  jmp print_loop

  jmp shell

.print_input:
  mov ah, 0x0e
  int 0x10
  inc cl

  mov [buffer + bx], al
  inc bx

  ret

.backspace:
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

  ret

.buffer_bounds:
  cmp cl, 0
  je .input
  jne .backspace

print:
  lodsb
  or al, al
  jz .end
  mov ah, 0x0e
  int 0x10
  jmp print

.end:
  ret

print_loop:
  mov al, [bx]
  cmp al, 0
  je .clear_buffer

  int 0x10
  inc bx

  jmp print_loop

.clear_buffer:
  mov al, 0
  mov [buffer + bx], al
  mov bx, 0

.null_char:
  inc bx
  cmp bx, 258
  je .end_print
  mov al, 0
  mov [buffer + bx], al
  jmp .null_char

.end_print:
  ret

cls:
  pusha
  mov ah, 0x00
  mov al, 0x03
  int 0x10
  popa
  ret

section .data
boot_msg:
  db 13, 10, "HighlightOS v0.0.1", 13, 10, 10, " Copyright (C) 2024", 13, 10, " Adam Perkowski", 0
nl:
  db 13, 10, 10, "hls <", 0

buffer times 258 db 0

times 510-($-$$) db 0
dw 0xaa55
