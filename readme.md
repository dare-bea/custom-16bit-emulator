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

- [x] [instructions.csv](instructions.csv).

### Addressing Modes

- `reg`   - A, B, C, D.
- `dst`   - A, B, C, D, SP, PC, FLAGS.
- `src`   - A, B, C, D, SP, PC, FLAGS.
- `#imm`  - 16-bit immediate value.
- `addr`  - 16-bit absolute address.
- `flags` - 8-bit mask.
- `rel`   - Signed 8-bit offset.

| Mnemonic    | Memory                       |
| ----------- | ---------------------------- |
| N/A         | `OOOO0000`                   |
| `rel`       | `OOOO0001 sxxxxxxx`          |
| `flags`     | `111X0001 ffffffff`          |
| `addr`      | `OOOO0010 llllllll hhhhhhhh` |
| `dst, src`  | `OOOO0011 dddsss00`          |
| `reg`       | `OOOO01rr`                   |
| `addr, reg` | `OOOO10rr llllllll hhhhhhhh` |
| `reg, #imm` | `OOOO11rr llllllll hhhhhhhh` |

## Memory Layout

0x10000 bytes of addressable memory (64KiB)

- 0x4000..0x6FFF: RAM
- 0x7F00..0x7FFF: Port In/Out
    - 0x7F00: Console In/Out
- 0x8000..0xFFFF: ROM
    - 0xFFE0..0xFFFF: Interrupt Vector Table
        - 0xFFFE: Reset Vector