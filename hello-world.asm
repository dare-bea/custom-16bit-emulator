main:
    LDI B, data
    CALL print
    HLT

print:
    LD B
    TEST A
    JZ print.end
    ST $7F00  ; Output to console port
    INC B
    JMP print
    print.end:
    RET

data:
    .ascii "Hello, World!"
    .db $0A $00