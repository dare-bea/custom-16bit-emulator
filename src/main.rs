//! ## Principles of Registers
//!
//! - Only A can read/write memory. Arithmetic operations may only mutate A.
//! - B is used for addressing. It is not used for memory access.
//! - C is used for loops. It is not used for memory access.
//! - D is used for port indexing. It is not used for memory access.
//!
//! The GPRs may be used for any arithmetic operation.

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
#[repr(u8)]
pub enum GeneralPurposeRegister {
    A,
    B,
    C,
    D,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum Instruction {
    /// Load the value of the given register into the accumulator.
    LoadFrom(GeneralPurposeRegister),

    /// Store the value of the accumulator to the given register.
    StoreTo(GeneralPurposeRegister),
    /// Zero the given register.
    Zero(GeneralPurposeRegister),
    /// Load the immediate value into the given register.
    LoadImmediate(GeneralPurposeRegister, u16),

    /// Load the value at the given address into the accumulator.
    LoadAddress(u16),
    /// Load the value at the address in the base register into the accumulator.
    LoadIndirect,
    /// Load the value at the given address relative to the base register into the accumulator.
    LoadRelative(u16),
    /// Load the value at the given address relative to the stack pointer into the accumulator.
    LoadStackRelative(u16),

    /// Store the lower byte of the accumulator to the given address.
    StoreByteAddress(u16),
    /// Store the lower byte of the accumulator to the address in the base register.
    StoreByteIndirect,
    /// Store the lower byte of the accumulator to the given address relative to the base register.
    StoreByteRelative(u16),
    /// Store the lower byte of the accumulator to the given address relative to the stack pointer.
    StoreByteStackRelative(u16),

    /// Store the value of the accumulator to the given address.
    StoreAddress(u16),
    /// Store the value of the accumulator to the address in the base register.
    StoreIndirect,
    /// Store the value of the accumulator to the given address relative to the base register.
    StoreRelative(u16),
    /// Store the value of the accumulator to the given address relative to the stack pointer.
    StoreStackRelative(u16),

    /// Invert the accumulator.
    Not,
    /// Increment the accumulator.
    Increment,
    /// Decrement the accumulator.
    Decrement,

    /// Bitwise AND the accumulator with the given register.
    And(GeneralPurposeRegister),
    /// Bitwise OR the accumulator with the given register.
    Or(GeneralPurposeRegister),
    /// Bitwise XOR the accumulator with the given register.
    Xor(GeneralPurposeRegister),
    /// Left shift the accumulator by the given register.
    LeftShift(GeneralPurposeRegister),
    /// Right shift the accumulator by the given register.
    RightShift(GeneralPurposeRegister),
    /// Add the given register to the accumulator.
    Add(GeneralPurposeRegister),
    /// Subtract the given register from the accumulator.
    Subtract(GeneralPurposeRegister),
    /// Add the given register to the accumulator with the carry flag.
    AddWithCarry(GeneralPurposeRegister),
    /// Subtract the given register from the accumulator with the carry flag.
    SubtractWithBorrow(GeneralPurposeRegister),

    /// Compare the accumulator with the given register.
    Compare(GeneralPurposeRegister),
    /// Compare the accumulator with the given immediate value.
    CompareImmediate(u16),

    /// Increment the counter register.
    IncrementC,
    /// Decrement the counter register.
    DecrementC,
    /// Compare the counter register with the given register.
    CompareC(GeneralPurposeRegister),
    /// Compare the counter register with the given immediate value.
    CompareCImmediate(u16),

    /// Jump to the given address.
    Jump(u16),
    /// Jump to the given address relative to the base register.
    JumpRelative(u16),

    /// Jump to the given address if the given condition is true.
    JumpIf(u8, u16),
    /// Jump to the given address relative to the base register if the given condition is true.
    JumpRelativeIf(u8, u16),

    /// Decrement the counter register and jump to the given address if the counter register is not zero.
    Loop(u16),
    /// Decrement the counter register and jump to the given address relative to the base register if the C register is not zero.
    LoopRelative(u16),

    /// Push the accumulator onto the stack.
    Push,
    /// Pop the accumulator from the stack.
    Pop,

    /// Push the program counter onto the stack.
    PushPC,
    /// Pop the program counter from the stack.
    PopPC,

    /// Push the flags onto the stack.
    PushFlags,
    /// Pop the flags from the stack.
    PopFlags,

    /// Read the port specified by the data register into the accumulator.
    Input,
    /// Write the accumulator to the port specified by the data register.
    Output,

    /// Set the interrupt vector to the given address.
    SetInterrupt(u16),
    /// Clear the given flag.
    Clear(u8),
    /// Set the given flag.
    Set(u8),
}

impl From<Instruction> for Vec<u8> {
    fn from(value: Instruction) -> Self {
        use Instruction::*;
        match value {
            LoadFrom(reg) => vec![0x00 | reg as u8],
            StoreTo(reg) => vec![0x04 | reg as u8],
            Zero(reg) => vec![0x08 | reg as u8],
            LoadImmediate(reg, value) => vec![0x0C | reg as u8, value as u8, (value >> 8) as u8],

            LoadAddress(address) => vec![0x10, address as u8, (address >> 8) as u8],
            LoadIndirect => vec![0x11],
            LoadRelative(offset) => vec![0x12, offset as u8, (offset >> 8) as u8],
            LoadStackRelative(offset) => vec![0x13, offset as u8, (offset >> 8) as u8],

            StoreAddress(address) => vec![0x18, address as u8, (address >> 8) as u8],
            StoreIndirect => vec![0x19],
            StoreRelative(offset) => vec![0x1A, offset as u8, (offset >> 8) as u8],
            StoreStackRelative(offset) => vec![0x1B, offset as u8, (offset >> 8) as u8],

            StoreByteAddress(address) => vec![0x1C, address as u8, (address >> 8) as u8],
            StoreByteIndirect => vec![0x1D],
            StoreByteRelative(offset) => vec![0x1E, offset as u8, (offset >> 8) as u8],
            StoreByteStackRelative(offset) => vec![0x1F, offset as u8, (offset >> 8) as u8],

            Not => vec![0x20],
            Increment => vec![0x21],
            Decrement => vec![0x22],
            And(reg) => vec![0x24 | reg as u8],
            Or(reg) => vec![0x28 | reg as u8],
            Xor(reg) => vec![0x2C | reg as u8],
            LeftShift(reg) => vec![0x30 | reg as u8],
            RightShift(reg) => vec![0x34 | reg as u8],
            Add(reg) => vec![0x38 | reg as u8],
            Subtract(reg) => vec![0x3C | reg as u8],
            AddWithCarry(reg) => vec![0x40 | reg as u8],
            SubtractWithBorrow(reg) => vec![0x44 | reg as u8],

            Compare(reg) => vec![0x48 | reg as u8],
            CompareImmediate(value) => vec![0x4C, value as u8, (value >> 8) as u8],

            IncrementC => vec![0x51],
            DecrementC => vec![0x52],
            CompareC(reg) => vec![0x54 | reg as u8],
            CompareCImmediate(value) => vec![0x58, value as u8, (value >> 8) as u8],

            Jump(address) => vec![0x60, address as u8, (address >> 8) as u8],
            Loop(address) => vec![0x68, address as u8, (address >> 8) as u8],
            JumpRelative(offset) => vec![0x70, offset as u8, (offset >> 8) as u8],
            LoopRelative(offset) => vec![0x78, offset as u8, (offset >> 8) as u8],

            JumpIf(cond, address) => vec![0x80 | cond as u8, address as u8, (address >> 8) as u8],
            JumpRelativeIf(cond, offset) => {
                vec![0x90 | cond as u8, offset as u8, (offset >> 8) as u8]
            },

            Push => vec![0xA0],
            PushPC => vec![0xA1],
            PushFlags => vec![0xA2],

            Pop => vec![0xA8],
            PopPC => vec![0xA9],
            PopFlags => vec![0xAA],

            Input => vec![0xB0],
            Output => vec![0xB1],

            SetInterrupt(address) => vec![0xD0, address as u8, (address >> 8) as u8],
            Clear(flag) => vec![0xE0 | flag as u8],
            Set(flag) => vec![0xF0 | flag as u8],
        }
    }
}

pub enum InstructionError {
    InvalidOpcode(u8),
    EndOfInput,
}

impl Instruction {
    pub fn from_iter<'a>(mut iter: impl Iterator<Item = &'a u8>) -> Result<(Self, u32), InstructionError> {
        use Instruction::*;
        let mut count = 0u32;

        let mut next_byte = || match iter.next() {
            Some(byte) => {
                count += 1;
                Ok(*byte)
            }
            None => Err(InstructionError::EndOfInput),
        };

        let opcode = next_byte()?;
        let register = match opcode & 3 {
            0 => GeneralPurposeRegister::A,
            1 => GeneralPurposeRegister::B,
            2 => GeneralPurposeRegister::C,
            3 => GeneralPurposeRegister::D,
            _ => unreachable!(),
        };
        let result = match opcode {
            0x00..=0x03 => LoadFrom(register),
            0x04..=0x07 => StoreTo(register),
            0x08..=0x0B => Zero(register),
            0x0C..=0x0F => LoadImmediate(register, u16::from_le_bytes([next_byte()?, next_byte()?])),
            0x10 => LoadAddress(u16::from_le_bytes([next_byte()?, next_byte()?])),
            0x11 => LoadIndirect,
            0x12 => LoadRelative(u16::from_le_bytes([next_byte()?, next_byte()?])),
            0x13 => LoadStackRelative(u16::from_le_bytes([next_byte()?, next_byte()?])),
            0x18 => StoreAddress(u16::from_le_bytes([next_byte()?, next_byte()?])),
            0x19 => StoreIndirect,
            0x1A => StoreRelative(u16::from_le_bytes([next_byte()?, next_byte()?])),
            0x1B => StoreStackRelative(u16::from_le_bytes([next_byte()?, next_byte()?])),
            0x1C => StoreByteAddress(u16::from_le_bytes([next_byte()?, next_byte()?])),
            0x1D => StoreByteIndirect,
            0x1E => StoreByteRelative(u16::from_le_bytes([next_byte()?, next_byte()?])),
            0x1F => StoreByteStackRelative(u16::from_le_bytes([next_byte()?, next_byte()?])),
            0x20 => Not,
            0x21 => Increment,
            0x22 => Decrement,
            0x24..=0x27 => And(register),
            0x28..=0x2B => Or(register),
            0x2C..=0x2F => Xor(register),
            0x30..=0x33 => LeftShift(register),
            0x34..=0x37 => RightShift(register),
            0x38..=0x3B => Add(register),
            0x3C..=0x3F => Subtract(register),
            0x40..=0x43 => AddWithCarry(register),
            0x44..=0x47 => SubtractWithBorrow(register),
            0x48..=0x4B => Compare(register),
            0x4C => CompareImmediate(u16::from_le_bytes([next_byte()?, next_byte()?])),
            0x51 => IncrementC,
            0x52 => DecrementC,
            0x54..=0x57 => CompareC(register),
            0x58 => CompareCImmediate(u16::from_le_bytes([next_byte()?, next_byte()?])),
            0x60 => Jump(u16::from_le_bytes([next_byte()?, next_byte()?])),
            0x68 => Loop(u16::from_le_bytes([next_byte()?, next_byte()?])),
            0x70 => JumpRelative(u16::from_le_bytes([next_byte()?, next_byte()?])),
            0x78 => LoopRelative(u16::from_le_bytes([next_byte()?, next_byte()?])),
            0x80..=0x8F => JumpIf(
                opcode & 0xF,
                u16::from_le_bytes([next_byte()?, next_byte()?]),
            ),
            0x90..=0x9F => JumpRelativeIf(
                opcode & 0xF,
                u16::from_le_bytes([next_byte()?, next_byte()?]),
            ),
            0xA0 => Push,
            0xA1 => PushPC,
            0xA2 => PushFlags,
            0xA8 => Pop,
            0xA9 => PopPC,
            0xAA => PopFlags,
            0xB0 => Input,
            0xB1 => Output,
            0xD0 => SetInterrupt(u16::from_le_bytes([next_byte()?, next_byte()?])),
            0xE0..=0xEF => Clear(opcode & 0xF),
            0xF0..=0xFF => Set(opcode & 0xF),

            _ => unimplemented!(),
        };
        Ok((result, count))
    }
}

pub mod condition {
    /// Zero flag is set. Equivalent to `[condition::EQUAL]`.
    pub const ZERO: u8 = 0;
    /// Zero flag is set. Equivalent to `[condition::ZERO]`.
    pub const EQUAL: u8 = 0;
    /// Sign flag is set.
    pub const SIGN: u8 = 1;
    /// Carry flag is set. Equivalent to `[condition::BELOW]`, `[condition::NOT_ABOVE_EQUAL]`.
    pub const CARRY: u8 = 2;
    /// Carry flag is set. Equivalent to `[condition::CARRY]`, `[condition::NOT_ABOVE_EQUAL]`.
    pub const BELOW: u8 = 2;
    /// Carry flag is set. Equivalent to `[condition::CARRY]`, `[condition::BELOW]`.
    pub const NOT_ABOVE_EQUAL: u8 = 2;
    /// Overflow flag is set.
    pub const OVERFLOW: u8 = 3;
    /// Carry or zero flag is set. Equivalent to `[condition::NOT_ABOVE]`.
    pub const BELOW_EQUAL: u8 = 5;
    /// Carry or zero flag is set. Equivalent to `[condition::BELOW_EQUAL]`.
    pub const NOT_ABOVE: u8 = 5;
    /// Zero flag is set or sign flag is not equal to overflow flag. Equivalent to `[condition::NOT_GREATER]`.
    pub const LESS_EQUAL: u8 = 6;
    /// Zero flag is set or sign flag is not equal to overflow flag. Equivalent to `[condition::LESS_EQUAL]`.
    pub const NOT_GREATER: u8 = 6;
    /// Sign flag is not equal to overflow flag. Equivalent to `[condition::NOT_GREATER_EQUAL]`.
    pub const LESS: u8 = 7;
    /// Sign flag is not equal to overflow flag. Equivalent to `[condition::LESS]`.
    pub const NOT_GREATER_EQUAL: u8 = 7;
    /// Zero flag is clear. Equivalent to `[condition::NOT_EQUAL]`.
    pub const NOT_ZERO: u8 = 8;
    /// Zero flag is clear. Equivalent to `[condition::NOT_ZERO]`.
    pub const NOT_EQUAL: u8 = 8;
    /// Sign flag is clear.
    pub const NOT_SIGN: u8 = 9;
    /// Carry flag is clear. Equivalent to `[condition::ABOVE]`, `[condition::NOT_BELOW_EQUAL]`.
    pub const NOT_CARRY: u8 = 10;
    /// Carry flag is clear. Equivalent to `[condition::NOT_CARRY]`, `[condition::NOT_BELOW_EQUAL]`.
    pub const ABOVE: u8 = 10;
    /// Carry flag is clear. Equivalent to `[condition::NOT_CARRY]`, `[condition::ABOVE]`.
    pub const NOT_BELOW_EQUAL: u8 = 10;
    /// Overflow flag is clear.
    pub const NOT_OVERFLOW: u8 = 11;
    /// Carry and zero flags are clear. Equivalent to `[condition::ABOVE_EQUAL]`.
    pub const NOT_BELOW: u8 = 13;
    /// Carry and zero flags are clear. Equivalent to `[condition::NOT_BELOW]`.
    pub const ABOVE_EQUAL: u8 = 13;
    /// Zero flag is clear or sign flag is equal to overflow flag. Equivalent to `[condition::GREATER]`.
    pub const NOT_LESS_EQUAL: u8 = 14;
    /// Zero flag is clear or sign flag is equal to overflow flag. Equivalent to `[condition::NOT_LESS_EQUAL]`.
    pub const GREATER: u8 = 14;
    /// Sign flag is equal to overflow flag. Equivalent to `[condition::GREATER_EQUAL]`.
    pub const NOT_LESS: u8 = 15;
    /// Sign flag is equal to overflow flag. Equivalent to `[condition::NOT_LESS]`.
    pub const GREATER_EQUAL: u8 = 15;
}

pub mod flag {
    pub const ZERO: u8 = 0;
    pub const SIGN: u8 = 1;
    pub const CARRY: u8 = 2;
    pub const OVERFLOW: u8 = 3;
    pub const INTERRUPT: u8 = 14;
    pub const HALT: u8 = 15;
}

const MEM_SIZE: usize = 0x10000;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Emulator {
    /// Accumulator (operations)
    a: u16,
    /// Base (addresses)
    b: u16,
    /// Counter (loops)
    c: u16,
    /// Data (ports)
    d: u16,
    /// Program Counter
    pc: u16,
    /// Stack Pointer
    sp: u16,
    /// Program Flags
    flags: u16,
    /// Program Memory
    memory: [u8; MEM_SIZE],
}

impl Emulator {
    pub fn new<IterableBytes: IntoIterator<Item = u8>>(memory: IterableBytes) -> Self {
        Self {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            pc: 0,
            sp: 0,
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

    pub fn register_a(&self) -> u16 {
        self.a
    }
    pub fn register_b(&self) -> u16 {
        self.b
    }
    pub fn register_c(&self) -> u16 {
        self.c
    }
    pub fn register_d(&self) -> u16 {
        self.d
    }
    pub fn register_pc(&self) -> u16 {
        self.pc
    }
    pub fn register_sp(&self) -> u16 {
        self.sp
    }
    pub fn flags(&self) -> u16 {
        self.flags
    }
    pub fn memory(&self) -> &[u8] {
        &self.memory
    }

    pub fn register(&self, reg: GeneralPurposeRegister) -> u16 {
        match reg {
            GeneralPurposeRegister::A => self.a,
            GeneralPurposeRegister::B => self.b,
            GeneralPurposeRegister::C => self.c,
            GeneralPurposeRegister::D => self.d,
        }
    }

    fn mut_register(&mut self, reg: GeneralPurposeRegister) -> &mut u16 {
        match reg {
            GeneralPurposeRegister::A => &mut self.a,
            GeneralPurposeRegister::B => &mut self.b,
            GeneralPurposeRegister::C => &mut self.c,
            GeneralPurposeRegister::D => &mut self.d,
        }
    }
    
    pub fn advance(&mut self) {
        match Instruction::from_iter(self.memory.iter().cycle()) {
            Ok((instruction, count)) => {
                self.pc = self.pc.wrapping_add(count as u16);
                self.execute(instruction);
            },
            Err(InstructionError::EndOfInput) => {
                unreachable!("Should not be able to reach end of input since we are repeating the memory")
            },
            Err(InstructionError::InvalidOpcode(opcode)) => {
                panic!("Invalid opcode: {}", opcode);
            }
        }
    }

    pub fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::LoadFrom(reg) => { self.a = self.register(reg) }
            Instruction::StoreTo(reg) => { *self.mut_register(reg) = self.a }
            Instruction::Zero(reg) => { *self.mut_register(reg) = 0 }
            Instruction::LoadImmediate(reg, value) => { *self.mut_register(reg) = value }
            Instruction::LoadAddress(address) => { self.a = u16::from_le_bytes([self.memory[address as usize], self.memory[address as usize + 1]]) }
            Instruction::LoadIndirect => { self.a = u16::from_le_bytes([self.memory[self.b as usize], self.memory[self.b as usize + 1]]) }
            Instruction::LoadRelative(offset) => { self.a = u16::from_le_bytes([self.memory[(self.b + offset) as usize], self.memory[(self.b + offset) as usize + 1]]) }
            Instruction::LoadStackRelative(offset) => { self.a = u16::from_le_bytes([self.memory[(self.sp + offset) as usize], self.memory[(self.sp + offset) as usize + 1]]) }
            Instruction::StoreAddress(address) => { self.memory[address as usize] = self.a as u8; self.memory[address as usize + 1] = (self.a >> 8) as u8 }
            Instruction::StoreIndirect => { self.memory[self.b as usize] = self.a as u8; self.memory[self.b as usize + 1] = (self.a >> 8) as u8 }
            Instruction::StoreRelative(offset) => { self.memory[(self.b + offset) as usize] = self.a as u8; self.memory[(self.b + offset) as usize + 1] = (self.a >> 8) as u8 }
            Instruction::StoreStackRelative(offset) => { self.memory[(self.sp + offset) as usize] = self.a as u8; self.memory[(self.sp + offset) as usize + 1] = (self.a >> 8) as u8 }
            Instruction::StoreByteAddress(address) => { self.memory[address as usize] = self.a as u8 }
            Instruction::StoreByteIndirect => { self.memory[self.b as usize] = self.a as u8 }
            Instruction::StoreByteRelative(offset) => { self.memory[(self.b + offset) as usize] = self.a as u8 }
            Instruction::StoreByteStackRelative(offset) => { self.memory[(self.sp + offset) as usize] = self.a as u8 }
            Instruction::Not => { self.a = !self.a; self.set_operation_flags(self.a); }
            Instruction::Increment => {
                let (result, carry) = self.a.overflowing_add(1);
                let overflow = (self.a as i16).overflowing_add(1).1;
                self.a = result;
                self.set_operation_flags(self.a);
                self.flags |= (overflow as u16) << flag::OVERFLOW | (carry as u16) << flag::CARRY;
            }
             Instruction::Decrement => {
                 let (result, carry) = self.a.overflowing_sub(1);
                 let overflow = (self.a as i16).overflowing_sub(1).1;
                 self.a = result;
                 self.set_operation_flags(self.a);
                 self.flags |= (overflow as u16) << flag::OVERFLOW | (carry as u16) << flag::CARRY;
             }
             Instruction::And(reg) => { self.a &= self.register(reg); self.set_operation_flags(self.a); }
             Instruction::Or(reg) => { self.a |= self.register(reg); self.set_operation_flags(self.a); }
             Instruction::Xor(reg) => { self.a ^= self.register(reg); self.set_operation_flags(self.a); }
             Instruction::LeftShift(reg) => {
                 let shift = self.register(reg) as u32;
                 let carry = (self.a as u32) >> (16 - shift);
                 self.a <<= shift;
                 self.set_operation_flags(self.a);
                 self.flags |= (carry as u16) << flag::CARRY;
             }
             Instruction::RightShift(reg) => {
                 let shift = self.register(reg) as u32;
                 let carry = (self.a as u32) >> (shift - 1);
                 self.a >>= shift;
                 self.set_operation_flags(self.a);
                 self.flags |= (carry as u16) << flag::CARRY;
             }
             Instruction::Add(reg) => {
                 let (result, carry) = self.a.overflowing_add(self.register(reg));
                 let overflow = (self.a as i16).overflowing_add(self.register(reg) as i16).1;
                 self.a = result;
                 self.set_operation_flags(self.a);
                 self.flags |= (overflow as u16) << flag::OVERFLOW | (carry as u16) << flag::CARRY;
             }
             Instruction::Subtract(reg) => {
                 let (result, carry) = self.a.overflowing_sub(self.register(reg));
                 let overflow = (self.a as i16).overflowing_sub(self.register(reg) as i16).1;
                 self.a = result;
                 self.set_operation_flags(self.a);
                 self.flags |= (overflow as u16) << flag::OVERFLOW | (carry as u16) << flag::CARRY;
             }
             Instruction::AddWithCarry(reg) => {
                let (result, carry) = self.a.carrying_add(self.register(reg), self.flags & (1 << flag::CARRY) != 0);
                let overflow = (self.a as i16).carrying_add(self.register(reg) as i16, self.flags & (1 << flag::CARRY) != 0).1;
                 self.a = result;
                 self.set_operation_flags(self.a);
                 self.flags |= (overflow as u16) << flag::OVERFLOW | (carry as u16) << flag::CARRY;
             }
             Instruction::SubtractWithBorrow(reg) => {
                 let (result, carry) = self.a.borrowing_sub(self.register(reg), self.flags & (1 << flag::CARRY) != 0);
                 let overflow = (self.a as i16).borrowing_sub(self.register(reg) as i16, self.flags & (1 << flag::CARRY) != 0).1;
                 self.a = result;
                 self.set_operation_flags(self.a);
                 self.flags |= (overflow as u16) << flag::OVERFLOW | (carry as u16) << flag::CARRY;
             }
             Instruction::Compare(reg) => {
                 let (result, carry) = self.a.overflowing_sub(self.register(reg));
                 let overflow = (self.a as i16).overflowing_sub(self.register(reg) as i16).1;
                 self.set_operation_flags(result);
                 self.flags |= (overflow as u16) << flag::OVERFLOW | (carry as u16) << flag::CARRY;
             }
             Instruction::CompareImmediate(value) => {
                 let (result, carry) = self.a.overflowing_sub(value);
                 let overflow = (self.a as i16).overflowing_sub(value as i16).1;
                 self.set_operation_flags(result);
                 self.flags |= (overflow as u16) << flag::OVERFLOW | (carry as u16) << flag::CARRY;
             }
             Instruction::IncrementC => {
                 let (result, carry) = self.c.overflowing_add(1);
                 let overflow = (self.c as i16).overflowing_add(1).1;
                 self.c = result;
                 self.set_operation_flags(self.c);
                 self.flags |= (overflow as u16) << flag::OVERFLOW | (carry as u16) << flag::CARRY;
             }
             Instruction::DecrementC => {
                 let (result, carry) = self.c.overflowing_sub(1);
                 let overflow = (self.c as i16).overflowing_sub(1).1;
                 self.c = result;
                 self.set_operation_flags(self.c);
             }
             Instruction::CompareC(reg) => {
                 let (result, carry) = self.c.overflowing_sub(self.register(reg));
                 let overflow = (self.c as i16).overflowing_sub(self.register(reg) as i16).1;
                 self.set_operation_flags(result);
                 self.flags |= (overflow as u16) << flag::OVERFLOW | (carry as u16) << flag::CARRY;
             }
             Instruction::CompareCImmediate(value) => {
                 let (result, carry) = self.c.overflowing_sub(value);
                 let overflow = (self.c as i16).overflowing_sub(value as i16).1;
                 self.set_operation_flags(result);
                 self.flags |= (overflow as u16) << flag::OVERFLOW | (carry as u16) << flag::CARRY;
             }
             Instruction::Jump(address) => { self.pc = address }
             Instruction::JumpRelative(offset) => { self.pc = self.b.wrapping_add(offset) }
             Instruction::JumpIf(cond, address) => { if self.check_condition(cond) { self.pc = address } }
             Instruction::JumpRelativeIf(cond, offset) => { if self.check_condition(cond) { self.pc = self.b.wrapping_add(offset) } }
             Instruction::Loop(address) => { self.c = self.c.wrapping_sub(1); if self.c != 0 { self.pc = address } }
             Instruction::LoopRelative(offset) => { self.c = self.c.wrapping_sub(1); if self.c != 0 { self.pc = self.b.wrapping_add(offset) } }
             Instruction::Push => { self.sp = self.sp.wrapping_sub(2); self.memory[self.sp as usize] = self.a as u8; self.memory[self.sp as usize + 1] = (self.a >> 8) as u8 }
             Instruction::PushPC => { self.sp = self.sp.wrapping_sub(2); self.memory[self.sp as usize] = self.pc as u8; self.memory[self.sp as usize + 1] = (self.pc >> 8) as u8}
             Instruction::PushFlags => { self.sp = self.sp.wrapping_sub(2); self.memory[self.sp as usize] = self.flags as u8; self.memory[self.sp as usize + 1] = (self.flags >> 8) as u8 }
             Instruction::Pop => { self.a = u16::from_le_bytes([self.memory[self.sp as usize], self.memory[self.sp as usize + 1]]); self.sp = self.sp.wrapping_add(2) }
             Instruction::PopPC => { self.pc = u16::from_le_bytes([self.memory[self.sp as usize], self.memory[self.sp as usize + 1]]); self.sp = self.sp.wrapping_add(2) }
             Instruction::PopFlags => { self.flags = u16::from_le_bytes([self.memory[self.sp as usize], self.memory[self.sp as usize + 1]]); self.sp = self.sp.wrapping_add(2) }
             Instruction::Input => { self.a = self.memory[self.d as usize] as u16 }
             Instruction::Output => { self.memory[self.d as usize] = self.a as u8 }
             Instruction::SetInterrupt(address) => { self.memory[0xFFFE] = address as u8; self.memory[0xFFFF] = (address >> 8) as u8 }
             Instruction::Clear(flag) => { self.flags &= !(1 << flag) }
             Instruction::Set(flag) => { self.flags |= 1 << flag }
        }
    }
    
    fn set_operation_flags(&mut self, value: u16) {
        self.flags &= !(1 << flag::ZERO | 1 << flag::SIGN | 1 << flag::CARRY | 1 << flag::OVERFLOW);
        if value == 0 {
            self.flags |= 1 << flag::ZERO;
        }
        if value & 0x8000 != 0 {
            self.flags |= 1 << flag::SIGN;
        }
    }

    pub fn check_condition(&self, cond: u8) -> bool {
        use condition::*;
        match cond {
            ZERO | EQUAL => self.flags & (1 << flag::ZERO) != 0,
            SIGN => self.flags & (1 << flag::SIGN) != 0,
            CARRY | BELOW | NOT_ABOVE_EQUAL => self.flags & (1 << flag::CARRY) != 0,
            OVERFLOW => self.flags & (1 << flag::OVERFLOW) != 0,
            BELOW_EQUAL | NOT_ABOVE => self.flags & (1 << flag::CARRY | 1 << flag::ZERO) != 0,
            LESS_EQUAL | NOT_GREATER => self.flags & (1 << flag::ZERO) != 0 || self.flags & (1 << flag::SIGN) != self.flags & (1 << flag::OVERFLOW),
            LESS | NOT_GREATER_EQUAL => self.flags & (1 << flag::SIGN) != self.flags & (1 << flag::OVERFLOW),
            NOT_ZERO | NOT_EQUAL => self.flags & (1 << flag::ZERO) == 0,
            NOT_SIGN => self.flags & (1 << flag::SIGN) == 0,
            NOT_CARRY | ABOVE | NOT_BELOW_EQUAL => self.flags & (1 << flag::CARRY) == 0,
            NOT_OVERFLOW => self.flags & (1 << flag::OVERFLOW) == 0,
            NOT_BELOW | ABOVE_EQUAL => self.flags & (1 << flag::CARRY | 1 << flag::ZERO) == 0,
            NOT_LESS_EQUAL | GREATER => self.flags & (1 << flag::ZERO) == 0 && self.flags & (1 << flag::SIGN) == self.flags & (1 << flag::OVERFLOW),
            NOT_LESS | GREATER_EQUAL => self.flags & (1 << flag::SIGN) == self.flags & (1 << flag::OVERFLOW),
            _ => unimplemented!("Invalid condition: {cond}"),
        }
    }
}

fn main() {
    let mut emu = Emulator::new([]);
    println!("{:?}", emu.advance());
}
