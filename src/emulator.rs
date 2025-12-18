use crate::flag;
use crate::isa::{Instruction, InstructionError};
use crate::memory::Memory;
use crate::register::GeneralPurposeRegister;

pub const MEM_SIZE: usize = 0x10000;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Emulator<M: Memory = [u8; MEM_SIZE]> {
    /// Accumulator (operations)
    pub a: u16,
    /// Base (addresses)
    pub b: u16,
    /// Counter (loops)
    pub c: u16,
    /// Data (ports)
    pub d: u16,
    /// Program Counter
    pub pc: u16,
    /// Stack Pointer
    pub sp: u16,
    /// Program Flags
    pub flags: u16,
    /// Program Memory
    pub memory: M,
}

impl<M: Memory> Emulator<M> {
    pub fn new(memory: M) -> Self {
        Self {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            pc: 0,
            sp: 0xF000,
            flags: 0,
            memory,
        }
    }

    pub fn register(&self, reg: GeneralPurposeRegister) -> u16 {
        match reg {
            GeneralPurposeRegister::A => self.a,
            GeneralPurposeRegister::B => self.b,
            GeneralPurposeRegister::C => self.c,
            GeneralPurposeRegister::D => self.d,
        }
    }

    pub fn mut_register(&mut self, reg: GeneralPurposeRegister) -> &mut u16 {
        match reg {
            GeneralPurposeRegister::A => &mut self.a,
            GeneralPurposeRegister::B => &mut self.b,
            GeneralPurposeRegister::C => &mut self.c,
            GeneralPurposeRegister::D => &mut self.d,
        }
    }

    pub fn next_instruction(&self) -> Result<(Instruction, u32), InstructionError> {
        Instruction::try_from_iter(self.memory.read_array::<3>(self.pc as usize).iter())
    }

    pub fn advance(&mut self) {
        let (instruction, count) = self.next_instruction().unwrap();
        self.pc = self.pc.wrapping_add(count as u16);
        self.execute(instruction);
    }

    pub fn set_operation_flags(&mut self, value: u16) {
        self.flags &= !(1 << flag::ZERO | 1 << flag::SIGN | 1 << flag::CARRY | 1 << flag::OVERFLOW);
        if value == 0 {
            self.flags |= 1 << flag::ZERO;
        }
        if value & 0x8000 != 0 {
            self.flags |= 1 << flag::SIGN;
        }
    }

    pub fn check_condition(&self, cond: u8) -> bool {
        use crate::condition::*;

        #[allow(unreachable_patterns)]
        match cond {
            ZERO | EQUAL => self.flags & (1 << flag::ZERO) != 0,
            SIGN => self.flags & (1 << flag::SIGN) != 0,
            CARRY | BELOW | NOT_ABOVE_EQUAL => self.flags & (1 << flag::CARRY) != 0,
            OVERFLOW => self.flags & (1 << flag::OVERFLOW) != 0,
            BELOW_EQUAL | NOT_ABOVE => self.flags & (1 << flag::CARRY | 1 << flag::ZERO) != 0,
            LESS_EQUAL | NOT_GREATER => {
                self.flags & (1 << flag::ZERO) != 0
                    || self.flags & (1 << flag::SIGN) != self.flags & (1 << flag::OVERFLOW)
            }
            LESS | NOT_GREATER_EQUAL => {
                self.flags & (1 << flag::SIGN) != self.flags & (1 << flag::OVERFLOW)
            }
            NOT_ZERO | NOT_EQUAL => self.flags & (1 << flag::ZERO) == 0,
            NOT_SIGN => self.flags & (1 << flag::SIGN) == 0,
            NOT_CARRY | ABOVE | NOT_BELOW_EQUAL => self.flags & (1 << flag::CARRY) == 0,
            NOT_OVERFLOW => self.flags & (1 << flag::OVERFLOW) == 0,
            NOT_BELOW | ABOVE_EQUAL => self.flags & (1 << flag::CARRY | 1 << flag::ZERO) == 0,
            NOT_LESS_EQUAL | GREATER => {
                self.flags & (1 << flag::ZERO) == 0
                    && self.flags & (1 << flag::SIGN) == self.flags & (1 << flag::OVERFLOW)
            }
            NOT_LESS | GREATER_EQUAL => {
                self.flags & (1 << flag::SIGN) == self.flags & (1 << flag::OVERFLOW)
            }
            _ => unimplemented!("Invalid condition: {cond}"),
        }
    }
}

impl<M: Memory + std::default::Default> std::default::Default for Emulator<M> {
    fn default() -> Self {
        Self::new(M::default())
    }
}
