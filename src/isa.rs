use crate::flag;
use crate::register::Register;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum Instruction {
    LoadImmediate(Register, u16),
    LoadAddressAbsolute(u16),
    LoadAddressStackOffset(i8),
    LoadWordAbsolute(u16),
    LoadWordStackOffset(i8),
    LoadAddressIndirect(u16, Register),
    LoadWordIndirect(u16, Register),
    StoreAddressAbsolute(u16),
    StoreAddressStackOffset(i8),
    StoreWordAbsolute(u16),
    StoreWordStackOffset(i8),
    StoreAddressIndirect(u16, Register),
    StoreWordIndirect(u16, Register),
    MoveRegister(Register, Register),
    MoveRegisterToSP(Register),
    MoveSPToRegister(Register),
    And(Register),
    Or(Register),
    Xor(Register),
    ShiftLeft(Register),
    ShiftRight(Register),
    Add(Register),
    Subtract(Register),
    RotateLeft(Register),
    RotateRight(Register),
    AddWithCarry(Register),
    SubtractWithBorrow(Register),
    Negate(Register),
    Not(Register),
    Increment(Register),
    Decrement(Register),
    Compare(Register),
    Test(Register),
    CompareImmediate(Register, u16),
    TestImmediate(Register, u16),
    JumpAbsolute(u16),
    JumpNear(i8),
    JumpStackOffset(i8),
    Call(u16),
    JumpIndirect(u16, Register),
    JumpIf(u8, u16),
    PushPC,
    PopPC,
    PushFlags,
    PopFlags,
    PushRegister(Register),
    PopRegister(Register),
    ClearInterruptRequest(u8),
    SetInterruptRequest(u8),
    WaitForInterrupt,
    ReturnFromInterrupt,
    ClearFlags(u8),
    SetFlags(u8),
}

impl Instruction {
    pub const NO_OPERATION: Self = Self::MoveRegister(Register::A, Register::A);

    pub const HALT: Self = Self::SetFlags(flag::HALT);
}

impl From<Instruction> for Vec<u8> {
    fn from(value: Instruction) -> Self {
        use Instruction::*;
        match value {
            LoadImmediate(reg, imm) => vec![(reg as u8), imm as u8, (imm >> 8) as u8],
            LoadAddressAbsolute(addr) => vec![0x04, addr as u8, (addr >> 8) as u8],
            LoadAddressStackOffset(offset) => vec![0x05, offset as u8],
            LoadWordAbsolute(addr) => vec![0x06, addr as u8, (addr >> 8) as u8],
            LoadWordStackOffset(offset) => vec![0x07, offset as u8],
            LoadAddressIndirect(addr, reg) => {
                vec![0x08 | (reg as u8), addr as u8, (addr >> 8) as u8]
            }
            LoadWordIndirect(addr, reg) => {
                vec![0x0C | (reg as u8), addr as u8, (addr >> 8) as u8]
            }
            StoreAddressAbsolute(addr) => vec![0x11, addr as u8, (addr >> 8) as u8],
            StoreAddressStackOffset(offset) => vec![0x12, offset as u8],
            StoreWordAbsolute(addr) => vec![0x15, addr as u8, (addr >> 8) as u8],
            StoreWordStackOffset(offset) => vec![0x16, offset as u8],
            StoreAddressIndirect(addr, reg) => {
                vec![0x18 | (reg as u8), addr as u8, (addr >> 8) as u8]
            }
            StoreWordIndirect(addr, reg) => {
                vec![0x1C | (reg as u8), addr as u8, (addr >> 8) as u8]
            }
            MoveRegister(dest, src) => vec![0x20 | ((dest as u8) << 2) | (src as u8)],
            MoveRegisterToSP(reg) => vec![0x30 | (reg as u8)],
            MoveSPToRegister(reg) => vec![0x34 | (reg as u8)],
            And(reg) => vec![0x40 | (reg as u8)],
            Or(reg) => vec![0x44 | (reg as u8)],
            Xor(reg) => vec![0x48 | (reg as u8)],
            ShiftLeft(reg) => vec![0x4C | (reg as u8)],
            ShiftRight(reg) => vec![0x50 | (reg as u8)],
            Add(reg) => vec![0x54 | (reg as u8)],
            Subtract(reg) => vec![0x58 | (reg as u8)],
            RotateLeft(reg) => vec![0x5C | (reg as u8)],
            RotateRight(reg) => vec![0x60 | (reg as u8)],
            AddWithCarry(reg) => vec![0x64 | (reg as u8)],
            SubtractWithBorrow(reg) => vec![0x68 | (reg as u8)],
            Negate(reg) => vec![0x6C | (reg as u8)],
            Not(reg) => vec![0x70 | (reg as u8)],
            Increment(reg) => vec![0x74 | (reg as u8)],
            Decrement(reg) => vec![0x78 | (reg as u8)],
            Compare(reg) => vec![0x7C | (reg as u8)],
            Test(reg) => vec![0x80 | (reg as u8)],
            CompareImmediate(reg, imm) => {
                vec![0xA8 | (reg as u8), imm as u8, (imm >> 8) as u8]
            }
            TestImmediate(reg, imm) => {
                vec![0xAC | (reg as u8), imm as u8, (imm >> 8) as u8]
            }
            JumpAbsolute(addr) => vec![0xC0, addr as u8, (addr >> 8) as u8],
            JumpNear(offset) => vec![0xC1, offset as u8],
            JumpStackOffset(offset) => vec![0xC2, offset as u8],
            Call(addr) => vec![0xC3, addr as u8, (addr >> 8) as u8],
            JumpIndirect(addr, reg) => {
                vec![0xC4 | (reg as u8), addr as u8, (addr >> 8) as u8]
            }
            JumpIf(cond, addr) => vec![0xD0 | (cond & 0xF), addr as u8, (addr >> 8) as u8],
            PushPC => vec![0xE4],
            PopPC => vec![0xE5],
            PushFlags => vec![0xE6],
            PopFlags => vec![0xE7],
            PushRegister(reg) => vec![0xE8 | (reg as u8)],
            PopRegister(reg) => vec![0xEC | (reg as u8)],
            ClearInterruptRequest(irq) => vec![0xF0, irq],
            SetInterruptRequest(irq) => vec![0xF1, irq],
            WaitForInterrupt => vec![0xF2],
            ReturnFromInterrupt => vec![0xF3],
            ClearFlags(flags) => vec![0xFE, flags],
            SetFlags(flags) => vec![0xFF, flags],
        }
    }
}

impl From<&Instruction> for Vec<u8> {
    fn from(value: &Instruction) -> Self {
        Vec::from(*value)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum InstructionError {
    InvalidOpcode(u8),
    EndOfInput,
}

impl Instruction {
    pub fn make_bytes(instructions: &[Result<Self, &[u8]>]) -> Vec<u8> {
        let mut result = Vec::new();
        for &instruction in instructions {
            match instruction {
                Ok(instruction) => result.extend_from_slice(&Vec::from(instruction)),
                Err(bytes) => result.extend_from_slice(bytes),
            }
        }
        result
    }

    fn next_byte(
        iter: &mut impl Iterator<Item = u8>,
        count: &mut u32,
    ) -> Result<u8, InstructionError> {
        match iter.next() {
            Some(byte) => {
                *count += 1;
                Ok(byte)
            }
            None => Err(InstructionError::EndOfInput),
        }
    }

    fn next_word(
        iter: &mut impl Iterator<Item = u8>,
        count: &mut u32,
    ) -> Result<u16, InstructionError> {
        let low = Self::next_byte(iter, count)?;
        let high = Self::next_byte(iter, count)?;
        Ok(u16::from_le_bytes([low, high]))
    }

    pub fn try_from_iter(
        iter: impl IntoIterator<Item = u8>,
    ) -> Result<(Self, u32), InstructionError> {
        use Instruction::*;
        let mut iter = iter.into_iter();
        let mut count = 0u32;

        let opcode = Self::next_byte(&mut iter, &mut count)?;
        let register = match opcode & 3 {
            0 => Register::A,
            1 => Register::B,
            2 => Register::C,
            3 => Register::D,
            _ => unreachable!(),
        };
        let result = match opcode {
            0x00..=0x03 => LoadImmediate(register, Self::next_word(&mut iter, &mut count)?),
            0x04 => LoadAddressAbsolute(Self::next_word(&mut iter, &mut count)?),
            0x05 => LoadAddressStackOffset(Self::next_byte(&mut iter, &mut count)? as i8),
            0x06 => LoadWordAbsolute(Self::next_word(&mut iter, &mut count)?),
            0x07 => LoadWordStackOffset(Self::next_byte(&mut iter, &mut count)? as i8),
            0x08..=0x0B => LoadAddressIndirect(Self::next_word(&mut iter, &mut count)?, register),
            0x0C..=0x0F => LoadWordIndirect(Self::next_word(&mut iter, &mut count)?, register),
            0x11 => StoreAddressAbsolute(Self::next_word(&mut iter, &mut count)?),
            0x12 => StoreAddressStackOffset(Self::next_byte(&mut iter, &mut count)? as i8),
            0x15 => StoreWordAbsolute(Self::next_word(&mut iter, &mut count)?),
            0x16 => StoreWordStackOffset(Self::next_byte(&mut iter, &mut count)? as i8),
            0x18..=0x1B => StoreAddressIndirect(Self::next_word(&mut iter, &mut count)?, register),
            0x1C..=0x1F => StoreWordIndirect(Self::next_word(&mut iter, &mut count)?, register),
            0x20..=0x2F => {
                let dest = match (opcode >> 2) & 3 {
                    0 => Register::A,
                    1 => Register::B,
                    2 => Register::C,
                    3 => Register::D,
                    _ => unreachable!(),
                };
                MoveRegister(dest, register)
            }
            0x30..=0x33 => MoveRegisterToSP(register),
            0x34..=0x37 => MoveSPToRegister(register),
            0x40..=0x83 => match opcode & 0xFC {
                0x40 => And(register),
                0x44 => Or(register),
                0x48 => Xor(register),
                0x4C => ShiftLeft(register),
                0x50 => ShiftRight(register),
                0x54 => Add(register),
                0x58 => Subtract(register),
                0x5C => RotateLeft(register),
                0x60 => RotateRight(register),
                0x64 => AddWithCarry(register),
                0x68 => SubtractWithBorrow(register),
                0x6C => Negate(register),
                0x70 => Not(register),
                0x74 => Increment(register),
                0x78 => Decrement(register),
                0x7C => Compare(register),
                0x80 => Test(register),
                _ => return Err(InstructionError::InvalidOpcode(opcode)),
            },
            0xA8..=0xAB => CompareImmediate(register, Self::next_word(&mut iter, &mut count)?),
            0xAC..=0xAF => TestImmediate(register, Self::next_word(&mut iter, &mut count)?),
            0xC0 => JumpAbsolute(Self::next_word(&mut iter, &mut count)?),
            0xC1 => JumpNear(Self::next_byte(&mut iter, &mut count)? as i8),
            0xC2 => JumpStackOffset(Self::next_byte(&mut iter, &mut count)? as i8),
            0xC3 => Call(Self::next_word(&mut iter, &mut count)?),
            0xC4..=0xC7 => JumpIndirect(Self::next_word(&mut iter, &mut count)?, register),
            0xD0..=0xDF => {
                let cond = opcode & 0x0F;
                JumpIf(cond, Self::next_word(&mut iter, &mut count)?)
            }
            0xE4 => PushPC,
            0xE5 => PopPC,
            0xE6 => PushFlags,
            0xE7 => PopFlags,
            0xE8..=0xEB => PushRegister(register),
            0xEC..=0xEF => PopRegister(register),
            0xF0 => ClearInterruptRequest(Self::next_byte(&mut iter, &mut count)?),
            0xF1 => SetInterruptRequest(Self::next_byte(&mut iter, &mut count)?),
            0xF2 => WaitForInterrupt,
            0xF3 => ReturnFromInterrupt,
            0xFE => ClearFlags(Self::next_byte(&mut iter, &mut count)?),
            0xFF => SetFlags(Self::next_byte(&mut iter, &mut count)?),
            _ => return Err(InstructionError::InvalidOpcode(opcode)),
        };
        Ok((result, count))
    }
}
