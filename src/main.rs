//! ## Principles of Registers
//!
//! - Only A can read/write memory. Arithmetic operations may only mutate A.
//! - B is used for addressing. It is not used for memory access.
//! - C is used for loops. It is not used for memory access.
//! - D is used for port indexing. It is not used for memory access.
//!
//! The GPRs may be used for any arithmetic operation.

use asm::condition;
use asm::emulator::Emulator;
use asm::flag;
use asm::isa::Instruction;
use asm::memory::Memory;
use asm::register::Register;

fn main() {
    use Instruction::*;
    use Register::*;

    let print_status: bool = false;

    let mut emu = Emulator::new();

    emu.memory.rom.unlock();

    emu.memory.rom.load(
        0x0000,
        &Instruction::make_bytes(&[
            /* $8000 */ Ok(LoadImmediate(B, 0xC000)),
            /* $8003 */ Ok(Call(0xA000)),
            /* $8006 */ Ok(SetFlags(1 << flag::HALT)),
        ]),
    );

    emu.memory.rom.load(
        0x2000,
        &Instruction::make_bytes(&[
            /* $A000 */ Ok(LoadAddressIndirect(0, B)),
            /* $A003 */ Ok(And(A)),
            /* $A004 */ Ok(JumpIf(condition::ZERO, 0xA00E)),
            /* $A007 */ Ok(StoreAddressAbsolute(0x6000)),
            /* $A00A */ Ok(Increment(B)),
            /* $A00B */ Ok(JumpAbsolute(0xA000)),
            /* $A00E */ Ok(PopPC),
        ]),
    );

    emu.memory.rom.load(
        0x4000,
        &Instruction::make_bytes(&[/* $C000 */ Err("Hello, World!\n\0".as_bytes())]),
    );

    emu.memory.rom.load(
        0x7FFE,
        &Instruction::make_bytes(&[Err(0x8000u16.to_le_bytes().as_ref())]),
    );

    emu.memory.rom.lock();
    emu.reset();
    if print_status {
        eprintln!("Initial CPU State: {:?}", emu.cpu);
    }
    while emu.is_running() {
        if print_status {
            eprint!("{:04x} ", emu.cpu.pc);
            eprint!("- {:?} ", emu.next_cpu_instruction());
        }
        emu.advance();
        if print_status {
            eprintln!("- {:?}", emu.cpu);
        }
    }
}
