# Custom 16-bit Console Emulator

An emulator for a custom 16-bit console.

## Registers

- [x] Accumulator Register: Used in computations and memory access
- [x] Base Register: Used in memory addressing
- [x] Counter Register: Used for loops
- [x] Data Register: Used for I/O ports
- [x] Program Counter
- [x] Stack Pointer
- [x] Flags
    - [x] Zero
    - [x] Sign
    - [x] Carry
    - [x] Overflow
    - [x] Interrupt
    - [x] Halt 

## Instruction Set

- [ ] [instructions.csv](instructions.csv).

## Memory Layout

0x10000 bytes of addressable memory (64KiB)

- 0x8000 - 0xFFFF: ROM
    - 0xFFE0 - 0xFFFF: Interrupt Vector Table
        - 0xFFE0: Reset Vector