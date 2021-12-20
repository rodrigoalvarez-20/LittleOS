global loader               ; Symbolo de entrada para ELF

MAGIC_NUMBER equ 0x1BADB002 ; Define la constante del numero magico
KERNEL_STACK_SIZE equ 4096          ; Stack Size in bytes
FLAGS equ 0x0               ; Banderas Multiboot
CHECKSUM equ -MAGIC_NUMBER  ; Calcular el checksum
                            ; (MN + CKS + Flags debe de ser 0)
section .text:              ; Inicio de la seccion de texto (codigo)
align 4                     ; El codigo debe de estar alineado 4 bytes
    dd MAGIC_NUMBER         ; Escribir el Numero Magico a codigo maquina
    dd FLAGS                ; Tambien las banderas
    dd CHECKSUM             ; Y el checksum

section .bss
align 4                             ; Alinear a 4 bytes
kernel_stack:                       ; La etiqueta apunta al inicio de la memoria
    resb KERNEL_STACK_SIZE          ; Reservar stack para el kernel

mov esp, kernel_stack + KERNEL_STACK_SIZE   ; Apuntar ESP al inicio de la stack (final de la memoria)
extern kmain
call kmain

loader:                     ; La etiqueta de "cargador" (Es el punto de entrada en el script del linker)
    mov eax, 0xCAFEBABE     ; Se pone el numero 0XCAFEBABE en el registro eax
.loop:
    jmp .loop               ; Ciclo infinito

