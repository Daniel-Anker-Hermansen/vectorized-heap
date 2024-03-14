data:
    dd 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0
mask:
    dd -1, -1, 0, 0, 0, -1, 0, -1

global _start
_start:
    vmovdqu ymm0, [rel data]
    vmovdqu ymm1, [rel mask]
    vcompressps ymm1, ymm0
    mov rax, 0
    mov rax, [rax]
