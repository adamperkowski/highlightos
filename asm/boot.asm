; Copyright 2025 Adam Perkowski
; Smol help from <franzageek>

org 0x7c00
bits 16

global _start

_start:
  call cls
  xor ax, ax
  mov ds, ax
  mov es, ax
  mov cx, ax          ;clear registers
  mov bx, boot_msg 
  call print          ;print the boot message

  jmp shell           ;jmp to the shell

  jmp $               ;failsafe - jmp to itself

shell:
  mov ah, 0xE
  mov al, '>'
  int 0x10            ;print a '>'

  mov bx, nl
  call print          ;print the shell prompt

.input:
    mov cl, 0         ;initialize character counter
    mov bx, buffer    ;move the buffer address into bx
.get_input_loop:
    mov ah, 0
    int 0x16          ;get keypress

    cmp al, 0x8       
    je .check_buffer  ;check if backspace

    cmp al, 0xD
    je .print_input   ;check if return key

    cmp al, 0x20
    jb .get_input_loop  ;check if special key

    cmp al, 0x7F
    jae .get_input_loop ;check if special key

    cmp cl, 255
    je .get_input_loop  ;check if buffer is full

    mov ah, 0xE
    int 0x10          ;print input character

    mov [bx], al      ;add input character to the buffer
    inc bx            ;make bx point to the next location of the buffer
    inc cl            ;increment character counter

    jmp .get_input_loop

.check_buffer:      
    cmp cl, 0
    je .get_input_loop  ;if buffer is empty jmp back to the input loop
    jne .backspace      ;if not, handle backspace

.backspace:
    dec bx            ;make bx point to the previous character of the buffer
    mov [bx], ch      ;remove it from the buffer
    dec cl            ;decrement character counter

    mov ah, 0xE
    mov al, 0x8
    int 0x10          ;move the cursor one character back 
    mov al, 0
    int 0x10          ;overwrite the last entered character with a blank character
    mov al, 0x8
    int 0x10          ;move the cursor one character back

    jmp .get_input_loop ;jmp back to the input loop

.print_input:
    mov ah, 0xE
    mov al, 0xA
    int 0x10          
    mov al, 0xD
    int 0x10          ;print newline

    mov bx, buffer    ;move buffer start address into bx
.print_input_loop:
    mov al, [bx]      

    cmp al, 0         ;check if current character is a NULL-terminator
    je shell          ;if yes, stop printing

    int 0x10          ;if not, print the character

    mov [bx], ch      ;clear current buffer location
    inc bx            ;make bx point to the next character of the buffer

    jmp .print_input_loop

print:
  mov ah, 0xE
.print_loop:  ;basically print_input_loop, but it doesnt clear the character after printing
    mov al, [bx]      

    cmp al, 0         ;check if current character is a NULL-terminator
    je back           ;if yes, stop printing

    int 0x10          ;if not, print the character
    inc bx            ;make bx point to the next character of the string

    jmp .print_loop 

back:
  ret                 ;return

cls:
  pusha

  mov ah, 0x00
  mov al, 0x03
  int 0x10            ;clear screen buffer

  popa

  ret                 ;return

boot_msg: db 13, 10, "HighlightOS v0.0.1", 13, 10, 10, " Copyright (C) 2025", 13, 10, " Adam Perkowski", 0
nl: db 13, 10, 10, "hls <", 0
buffer: times 255 db 0

times 510-($-$$) db 0
dw 0xaa55
