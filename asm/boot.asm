; Copyright 2024 Adam Perkowski
; Smol help from <franzageek>

org 0x7c00
bits 16

global _start

_start:
  call cls
  xor ax, ax
  mov ds, ax
  mov es, ax
  mov bx, boot_msg
  call print

  jmp shell

  jmp $


shell:
  mov ah, 0xE
  mov al, '>'
  int 0x10

  mov bx, nl
  call print

.input:
    mov cl, 0
    mov bx, buffer
.get_input_loop:
    mov ah, 0
    int 0x16

    cmp al, 0x8
    je .check_buffer

    cmp al, 0xD
    je .print_input

    cmp al, 0x20
    jb .get_input_loop

    cmp al, 0x7F
    jae .get_input_loop

    cmp cl, 255
    je .get_input_loop

    mov ah, 0xE
    int 0x10

    mov [bx], al
    inc bx
    inc cl

    jmp .get_input_loop

.check_buffer:
    cmp cl, 0
    je .get_input_loop
    jne .backspace

.backspace:
    dec bx
    mov ch, 0
    mov [bx], ch
    dec cl
    mov ah, 0xE
    mov al, 0x8
    int 0x10
    mov al, 0
    int 0x10
    mov al, 0x8
    int 0x10
    jmp .get_input_loop

.print_input:
    mov ah, 0xE
    mov al, 0xA
    int 0x10
    mov al, 0xD
    int 0x10
    mov bx, buffer
    mov ch, 0
.print_input_loop:
    mov al, [bx]
    cmp al, 0
    je .end_print_input
    int 0x10
    mov [bx], ch
    inc bx
    jmp .print_input_loop

.end_print_input:
    jmp shell

print:
  mov ah, 0xE
.print_loop:
    mov al, [bx]
    cmp al, 0
    je back
    int 0x10
    inc bx
    jmp .print_loop

back:
  ret

cls:
  pusha
  mov ah, 0x00
  mov al, 0x03
  int 0x10
  popa
  ret

boot_msg: db 13, 10, "HighlightOS v0.0.1", 13, 10, 10, " Copyright (C) 2024", 13, 10, " Adam Perkowski", 0
nl: db 13, 10, 10, "hls <", 0
buffer: times 255 db 0

times 510-($-$$) db 0
dw 0xaa55
