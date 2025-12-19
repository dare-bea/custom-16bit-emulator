'main:
    LDI B, 'data
    CALL 'print
    HLT

'print:
    LDA (B)
    TEST A
    JZ '.end
    STO A, (0x7F00)  ; Output to console port
    INC B
    JMP 'print
    '.end:
    RET

'data:
    .ascii "Hello, World!\n\0"