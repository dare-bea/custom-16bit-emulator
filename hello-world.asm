main:
    LDI B, data
    CALL print
    HALT

print:
    LDA [B]
    AND A
    JZ end
    OUT
    INC B
    JMP print
    end:
    RET

data:
    .ascii "Hello, World!\n\0"