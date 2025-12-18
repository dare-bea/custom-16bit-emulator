use crate::types::*;
use crate::isa::*;

const MEM_SIZE: usize = 0x10000;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Emulator {
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
    /// Program Memory Bank
    pub memory: [u8; MEM_SIZE],
}

impl Emulator {
    pub fn new<IterableBytes: IntoIterator<Item = u8>>(memory: IterableBytes) -> Self {
        Self {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            pc: 0,
            sp: 0xF000,
            flags: 0,
            memory: memory
                .into_iter()
                .chain(std::iter::repeat(0))
                .take(MEM_SIZE)
                .collect::<Vec<_>>()
                .try_into()
                .expect("Should have exactly MEM_SIZE elements"),
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
        Instruction::from_iter(self.memory.iter().cycle().skip(self.pc as usize))
    }

    pub fn advance(&mut self) {
        match self.next_instruction() {
            Ok((instruction, count)) => {
                self.pc = self.pc.wrapping_add(count as u16);
                self.execute(instruction);
            }
            Err(InstructionError::EndOfInput) => {
                unreachable!(
                    "Should not be able to reach end of input since we are repeating the memory"
                )
            }
            Err(InstructionError::InvalidOpcode(opcode)) => {
                panic!("Invalid opcode: {}", opcode);
            }
        }
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
        use crate::types::condition::*;
        use crate::types::flag;
        
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
