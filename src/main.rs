//! ## Principles of Registers
//!
//! - Only A can read/write memory. Arithmetic operations may only mutate A.
//! - B is used for addressing. It is not used for memory access.
//! - C is used for loops. It is not used for memory access.
//! - D is used for port indexing. It is not used for memory access.
//!
//! The GPRs may be used for any arithmetic operation.

use asm::condition;
use asm::emulator::{Emulator, MEM_SIZE};
use asm::flag;
use asm::isa::Instruction;
use asm::memory::Memory;
use asm::register::Register;

fn main() {
    use Register::*;
    use Instruction::*;

    let print_status: bool = false;

    let mut emu = Emulator::<[u8; MEM_SIZE]>::new([0; MEM_SIZE]);

    emu.memory.write_array(
        0x0000,
        &Instruction::make_bytes(&[
            /* $0000 */ Ok(LoadImmediate(B, 0x4000)),
            /* $0003 */ Ok(Call(0x2000)),
            /* $0006 */ Ok(Set(flag::HALT)),
        ]),
    );

    emu.memory.write_array(
        0x2000,
        &Instruction::make_bytes(&[
            /* $2000 */ Ok(LoadByteIndirect),
            /* $2001 */ Ok(And(A)),
            /* $2002 */ Ok(JumpRelativeIf(condition::ZERO, 5)),
            /* $2005 */ Ok(Output),
            /* $2006 */ Ok(Increment(B)),
            /* $2007 */ Ok(JumpRelative(-10i16 as u16)),
            /* $200A */ Ok(Return),
        ]),
    );

    emu.memory.write_array(
        0x4000,
        &Instruction::make_bytes(&[/* $4000 */ Err("Hello, World!\n\0".as_bytes())]),
    );

    while emu.flags & (1 << flag::HALT) == 0 {
        if print_status {
            eprintln!(
                "A: {:04X} | B: {:04X} | C: {:04X} | D: {:04X}  |  SP: {:04X}  |  FLAGS: {:016b}  |  PC: {:04X}  |  {:?}",
                emu.a,
                emu.b,
                emu.c,
                emu.d,
                emu.sp,
                emu.flags,
                emu.pc,
                emu.next_instruction()
            );
        }
        emu.advance();
    }
}
