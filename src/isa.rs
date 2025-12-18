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

    /// Load the word at the given address into the accumulator.
    LoadAddress(u16),
    /// Load the word at the address in the base register into the accumulator.
    LoadIndirect,
    /// Load the word at the given address relative to the base register into the accumulator.
    LoadOffset(u16),
    /// Load the word at the given address relative to the stack pointer into the accumulator.
    LoadStackOffset(u16),

    /// Load the byte at the given address into the accumulator.
    LoadByteAddress(u16),
    /// Load the byte at the address in the base register into the accumulator.
    LoadByteIndirect,
    /// Load the byte at the given address relative to the base register into the accumulator.
    LoadByteOffset(u16),
    /// Load the byte at the given address relative to the stack pointer into the accumulator.
    LoadByteStackOffset(u16),

    /// Store the lower byte of the accumulator to the given address.
    StoreByteAddress(u16),
    /// Store the lower byte of the accumulator to the address in the base register.
    StoreByteIndirect,
    /// Store the lower byte of the accumulator to the given address relative to the base register.
    StoreByteOffset(u16),
    /// Store the lower byte of the accumulator to the given address relative to the stack pointer.
    StoreByteStackOffset(u16),

    /// Store the value of the accumulator to the given address.
    StoreAddress(u16),
    /// Store the value of the accumulator to the address in the base register.
    StoreIndirect,
    /// Store the value of the accumulator to the given address relative to the base register.
    StoreOffset(u16),
    /// Store the value of the accumulator to the given address relative to the stack pointer.
    StoreStackOffset(u16),

    /// Invert the given register.
    Not(GeneralPurposeRegister),
    /// Increment the given register.
    Increment(GeneralPurposeRegister),
    /// Decrement the given register.
    Decrement(GeneralPurposeRegister),

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
    CompareA(GeneralPurposeRegister),
    /// Compare the given register with the given immediate value.
    CompareImmediate(GeneralPurposeRegister, u16),

    /// Jump to the given address.
    Jump(u16),
    /// Jump to the given address relative to the base register.
    JumpOffset(u16),
    /// Jump to the given address relative to the next instruction.
    JumpRelative(u16),

    /// Jump to the given address if the given condition is true.
    JumpIf(u8, u16),
    /// Jump to the given address relative to the base register if the given condition is true.
    JumpOffsetIf(u8, u16),
    /// Jump to the given address relative to the next instruction if the given condition is true.
    JumpRelativeIf(u8, u16),

    /// Decrement the counter register and jump to the given address if the counter register is not zero.
    Loop(u16),
    /// Decrement the counter register and jump to the given address relative to the base register if the counter register is not zero.
    LoopOffset(u16),
    /// Decrement the counter register and jump to the given address relative to the next instruction if the counter register is not zero.
    LoopRelative(u16),

    /// Call a subroutine at the given address.
    Call(u16),
    /// Call a subroutine at the given address relative to the base register.
    CallOffset(u16),
    /// Call a subroutine at the given address relative to the next instruction.
    CallRelative(u16),

    /// Push the accumulator onto the stack.
    Push,
    /// Pop the accumulator from the stack.
    Pop,

    /// Push the program counter onto the stack.
    PushPC,
    /// Pop the program counter from the stack. This is used to return from a subroutine.
    Return,

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
            LoadFrom(reg) => vec![(reg as u8)],
            StoreTo(reg) => vec![0x04 | reg as u8],
            Zero(reg) => vec![0x08 | reg as u8],
            LoadImmediate(reg, value) => vec![0x0C | reg as u8, value as u8, (value >> 8) as u8],

            LoadAddress(address) => vec![0x10, address as u8, (address >> 8) as u8],
            LoadIndirect => vec![0x11],
            LoadOffset(offset) => vec![0x12, offset as u8, (offset >> 8) as u8],
            LoadStackOffset(offset) => vec![0x13, offset as u8, (offset >> 8) as u8],

            LoadByteAddress(address) => vec![0x14, address as u8, (address >> 8) as u8],
            LoadByteIndirect => vec![0x15],
            LoadByteOffset(offset) => vec![0x16, offset as u8, (offset >> 8) as u8],
            LoadByteStackOffset(offset) => vec![0x17, offset as u8, (offset >> 8) as u8],

            StoreAddress(address) => vec![0x18, address as u8, (address >> 8) as u8],
            StoreIndirect => vec![0x19],
            StoreOffset(offset) => vec![0x1A, offset as u8, (offset >> 8) as u8],
            StoreStackOffset(offset) => vec![0x1B, offset as u8, (offset >> 8) as u8],

            StoreByteAddress(address) => vec![0x1C, address as u8, (address >> 8) as u8],
            StoreByteIndirect => vec![0x1D],
            StoreByteOffset(offset) => vec![0x1E, offset as u8, (offset >> 8) as u8],
            StoreByteStackOffset(offset) => vec![0x1F, offset as u8, (offset >> 8) as u8],

            Not(reg) => vec![0x20 | reg as u8],
            Increment(reg) => vec![0x28 | reg as u8],
            Decrement(reg) => vec![0x2C | reg as u8],
            And(reg) => vec![0x30 | reg as u8],
            Or(reg) => vec![0x34 | reg as u8],
            Xor(reg) => vec![0x38 | reg as u8],
            LeftShift(reg) => vec![0x3C | reg as u8],
            RightShift(reg) => vec![0x40 | reg as u8],
            Add(reg) => vec![0x44 | reg as u8],
            Subtract(reg) => vec![0x48 | reg as u8],
            AddWithCarry(reg) => vec![0x4C | reg as u8],
            SubtractWithBorrow(reg) => vec![0x50 | reg as u8],

            CompareA(reg) => vec![0x54 | reg as u8],
            CompareImmediate(reg, value) => vec![0x58 | reg as u8, value as u8, (value >> 8) as u8],

            Jump(address) => vec![0x60, address as u8, (address >> 8) as u8],
            JumpOffset(offset) => vec![0x61, offset as u8, (offset >> 8) as u8],
            JumpRelative(offset) => vec![0x62, offset as u8, (offset >> 8) as u8],
            Loop(address) => vec![0x64, address as u8, (address >> 8) as u8],
            LoopOffset(offset) => vec![0x65, offset as u8, (offset >> 8) as u8],
            LoopRelative(offset) => vec![0x66, offset as u8, (offset >> 8) as u8],
            Call(address) => vec![0x68, address as u8, (address >> 8) as u8],
            CallOffset(offset) => vec![0x69, offset as u8, (offset >> 8) as u8],
            CallRelative(offset) => vec![0x6A, offset as u8, (offset >> 8) as u8],

            JumpIf(cond, address) => vec![0x70 | cond, address as u8, (address >> 8) as u8],
            JumpOffsetIf(cond, offset) => {
                vec![0x80 | cond, offset as u8, (offset >> 8) as u8]
            }
            JumpRelativeIf(cond, offset) => {
                vec![0x90 | cond, offset as u8, (offset >> 8) as u8]
            }

            Push => vec![0xA0],
            PushPC => vec![0xA1],
            PushFlags => vec![0xA2],

            Pop => vec![0xA8],
            Return => vec![0xA9],
            PopFlags => vec![0xAA],

            Input => vec![0xB0],
            Output => vec![0xB1],

            SetInterrupt(address) => vec![0xD0, address as u8, (address >> 8) as u8],
            Clear(flag) => vec![0xE0 | flag],
            Set(flag) => vec![0xF0 | flag],
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum InstructionError {
    InvalidOpcode(u8),
    EndOfInput,
}

impl Instruction {
    pub fn make_bytes(Vec<Result<Self, &[u8]>>) -> Vec<u8> {
        let mut bytes = Vec::new();
        for instruction in instructions {
            match instruction {
                Ok(instruction) => bytes.extend_from_slice(&Vec::from(instruction)),
                Err(bytes) => bytes.extend_from_slice(bytes),
            }
        }
    }
    
    pub fn from_iter<'a>(
        mut iter: &[u8]>,
    ) -> Result<(Self, u32), InstructionError> {
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
            0x0C..=0x0F => {
                LoadImmediate(register, u16::from_le_bytes([next_byte()?, next_byte()?]))
            }
            0x10 => LoadAddress(u16::from_le_bytes([next_byte()?, next_byte()?])),
            0x11 => LoadIndirect,
            0x12 => LoadOffset(u16::from_le_bytes([next_byte()?, next_byte()?])),
            0x13 => LoadStackOffset(u16::from_le_bytes([next_byte()?, next_byte()?])),
            0x14 => LoadByteAddress(u16::from_le_bytes([next_byte()?, next_byte()?])),
            0x15 => LoadByteIndirect,
            0x16 => LoadByteOffset(u16::from_le_bytes([next_byte()?, next_byte()?])),
            0x17 => LoadByteStackOffset(u16::from_le_bytes([next_byte()?, next_byte()?])),
            0x18 => StoreAddress(u16::from_le_bytes([next_byte()?, next_byte()?])),
            0x19 => StoreIndirect,
            0x1A => StoreOffset(u16::from_le_bytes([next_byte()?, next_byte()?])),
            0x1B => StoreStackOffset(u16::from_le_bytes([next_byte()?, next_byte()?])),
            0x1C => StoreByteAddress(u16::from_le_bytes([next_byte()?, next_byte()?])),
            0x1D => StoreByteIndirect,
            0x1E => StoreByteOffset(u16::from_le_bytes([next_byte()?, next_byte()?])),
            0x1F => StoreByteStackOffset(u16::from_le_bytes([next_byte()?, next_byte()?])),
            0x20..=0x23 => Not(register),
            0x28..=0x2B => Increment(register),
            0x2C..=0x2F => Decrement(register),
            0x30..=0x33 => And(register),
            0x34..=0x37 => Or(register),
            0x38..=0x3B => Xor(register),
            0x3C..=0x3F => LeftShift(register),
            0x40..=0x43 => RightShift(register),
            0x44..=0x47 => Add(register),
            0x48..=0x4B => Subtract(register),
            0x4C..=0x4F => AddWithCarry(register),
            0x50..=0x53 => SubtractWithBorrow(register),
            0x54..=0x57 => CompareA(register),
            0x58..=0x5B => {
                CompareImmediate(register, u16::from_le_bytes([next_byte()?, next_byte()?]))
            }
            0x60 => Jump(u16::from_le_bytes([next_byte()?, next_byte()?])),
            0x61 => JumpOffset(u16::from_le_bytes([next_byte()?, next_byte()?])),
            0x62 => JumpRelative(u16::from_le_bytes([next_byte()?, next_byte()?])),
            0x64 => Loop(u16::from_le_bytes([next_byte()?, next_byte()?])),
            0x65 => LoopOffset(u16::from_le_bytes([next_byte()?, next_byte()?])),
            0x66 => LoopRelative(u16::from_le_bytes([next_byte()?, next_byte()?])),
            0x68 => Call(u16::from_le_bytes([next_byte()?, next_byte()?])),
            0x69 => CallOffset(u16::from_le_bytes([next_byte()?, next_byte()?])),
            0x6A => CallRelative(u16::from_le_bytes([next_byte()?, next_byte()?])),
            0x70..=0x7F => JumpIf(
                opcode & 0xF,
                u16::from_le_bytes([next_byte()?, next_byte()?]),
            ),
            0x80..=0x8F => JumpOffsetIf(
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
            0xA9 => Return,
            0xAA => PopFlags,
            0xB0 => Input,
            0xB1 => Output,
            0xD0 => SetInterrupt(u16::from_le_bytes([next_byte()?, next_byte()?])),
            0xE0..=0xEF => Clear(opcode & 0xF),
            0xF0..=0xFF => Set(opcode & 0xF),

            _ => return Err(InstructionError::InvalidOpcode(opcode)),
        };
        Ok((result, count))
    }
}

impl Emulator {
pub fn execute(&mut self, instruction: Instruction) {
    match instruction {
        Instruction::LoadFrom(reg) => self.a = self.register(reg),
        Instruction::StoreTo(reg) => *self.mut_register(reg) = self.a,
        Instruction::Zero(reg) => *self.mut_register(reg) = 0,
        Instruction::LoadImmediate(reg, value) => *self.mut_register(reg) = value,
        Instruction::LoadAddress(address) => {
            self.a = u16::from_le_bytes([
                self.memory[address as usize],
                self.memory[address.wrapping_add(1) as usize],
            ])
        }
        Instruction::LoadIndirect => {
            self.a = u16::from_le_bytes([
                self.memory[self.b as usize],
                self.memory[self.b.wrapping_add(1) as usize],
            ])
        }
        Instruction::LoadOffset(offset) => {
            self.a = u16::from_le_bytes([
                self.memory[self.b.wrapping_add(offset) as usize],
                self.memory[self.b.wrapping_add(offset).wrapping_add(1) as usize],
            ])
        }
        Instruction::LoadStackOffset(offset) => {
            self.a = u16::from_le_bytes([
                self.memory[self.sp.wrapping_add(offset) as usize],
                self.memory[self.sp.wrapping_add(offset).wrapping_add(1) as usize],
            ])
        }
        Instruction::LoadByteAddress(address) => self.a = self.memory[address as usize] as u16,
        Instruction::LoadByteIndirect => self.a = self.memory[self.b as usize] as u16,
        Instruction::LoadByteOffset(offset) => {
            self.a = self.memory[self.b.wrapping_add(offset) as usize] as u16
        }
        Instruction::LoadByteStackOffset(offset) => {
            self.a = self.memory[self.sp.wrapping_add(offset) as usize] as u16
        }
        Instruction::StoreAddress(address) => {
            self.memory[address as usize] = self.a as u8;
            self.memory[address.wrapping_add(1) as usize] = (self.a >> 8) as u8
        }
        Instruction::StoreIndirect => {
            self.memory[self.b as usize] = self.a as u8;
            self.memory[self.b.wrapping_add(1) as usize] = (self.a >> 8) as u8
        }
        Instruction::StoreOffset(offset) => {
            self.memory[self.b.wrapping_add(offset) as usize] = self.a as u8;
            self.memory[self.b.wrapping_add(offset).wrapping_add(1) as usize] = (self.a >> 8) as u8
        }
        Instruction::StoreStackOffset(offset) => {
            self.memory[self.sp.wrapping_add(offset) as usize] = self.a as u8;
            self.memory[self.sp.wrapping_add(offset).wrapping_add(1) as usize] = (self.a >> 8) as u8
        }
        Instruction::StoreByteAddress(address) => self.memory[address as usize] = self.a as u8,
        Instruction::StoreByteIndirect => self.memory[self.b as usize] = self.a as u8,
        Instruction::StoreByteOffset(offset) => {
            self.memory[self.b.wrapping_add(offset) as usize] = self.a as u8
        }
        Instruction::StoreByteStackOffset(offset) => {
            self.memory[self.sp.wrapping_add(offset) as usize] = self.a as u8
        }
        Instruction::Not(reg) => {
            *self.mut_register(reg) = !self.register(reg);
            self.set_operation_flags(self.register(reg));
        }
        Instruction::Increment(reg) => {
            let (result, carry) = self.register(reg).overflowing_add(1);
            let overflow = (self.register(reg) as i16).overflowing_add(1).1;
            *self.mut_register(reg) = result;
            self.set_operation_flags(self.register(reg));
            self.flags |= (overflow as u16) << flag::OVERFLOW | (carry as u16) << flag::CARRY;
        }
        Instruction::Decrement(reg) => {
            let (result, carry) = self.register(reg).overflowing_sub(1);
            let overflow = (self.register(reg) as i16).overflowing_sub(1).1;
            *self.mut_register(reg) = result;
            self.set_operation_flags(self.register(reg));
            self.flags |= (overflow as u16) << flag::OVERFLOW | (carry as u16) << flag::CARRY;
        }
        Instruction::And(reg) => {
            self.a &= self.register(reg);
            self.set_operation_flags(self.a);
        }
        Instruction::Or(reg) => {
            self.a |= self.register(reg);
            self.set_operation_flags(self.a);
        }
        Instruction::Xor(reg) => {
            self.a ^= self.register(reg);
            self.set_operation_flags(self.a);
        }
        Instruction::LeftShift(reg) => {
            let (result, carry) = self.a.overflowing_shl(self.register(reg) as u32);
            self.a = result;
            self.set_operation_flags(self.a);
            self.flags |= (carry as u16) << flag::CARRY;
        }
        Instruction::RightShift(reg) => {
            let (result, carry) = self.a.overflowing_shr(self.register(reg) as u32);
            self.a = result;
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
            let (result, carry) = self
                .a
                .carrying_add(self.register(reg), self.flags & (1 << flag::CARRY) != 0);
            let overflow = (self.a as i16)
                .carrying_add(
                    self.register(reg) as i16,
                    self.flags & (1 << flag::CARRY) != 0,
                )
                .1;
            self.a = result;
            self.set_operation_flags(self.a);
            self.flags |= (overflow as u16) << flag::OVERFLOW | (carry as u16) << flag::CARRY;
        }
        Instruction::SubtractWithBorrow(reg) => {
            let (result, carry) = self
                .a
                .borrowing_sub(self.register(reg), self.flags & (1 << flag::CARRY) != 0);
            let overflow = (self.a as i16)
                .borrowing_sub(
                    self.register(reg) as i16,
                    self.flags & (1 << flag::CARRY) != 0,
                )
                .1;
            self.a = result;
            self.set_operation_flags(self.a);
            self.flags |= (overflow as u16) << flag::OVERFLOW | (carry as u16) << flag::CARRY;
        }
        Instruction::CompareA(reg) => {
            let (result, carry) = self.a.overflowing_sub(self.register(reg));
            let overflow = (self.a as i16).overflowing_sub(self.register(reg) as i16).1;
            self.set_operation_flags(result);
            self.flags |= (overflow as u16) << flag::OVERFLOW | (carry as u16) << flag::CARRY;
        }
        Instruction::CompareImmediate(reg, value) => {
            let (result, carry) = self.register(reg).overflowing_sub(value);
            let overflow = (self.register(reg) as i16).overflowing_sub(value as i16).1;
            self.set_operation_flags(result);
            self.flags |= (overflow as u16) << flag::OVERFLOW | (carry as u16) << flag::CARRY;
        }
        Instruction::Jump(address) => self.pc = address,
        Instruction::JumpOffset(offset) => self.pc = self.b.wrapping_add(offset),
        Instruction::JumpRelative(offset) => self.pc = self.pc.wrapping_add(offset),
        Instruction::JumpIf(cond, address) => {
            if self.check_condition(cond) {
                self.pc = address
            }
        }
        Instruction::JumpOffsetIf(cond, offset) => {
            if self.check_condition(cond) {
                self.pc = self.b.wrapping_add(offset)
            }
        }
        Instruction::JumpRelativeIf(cond, offset) => {
            if self.check_condition(cond) {
                self.pc = self.pc.wrapping_add(offset)
            }
        }
        Instruction::Loop(address) => {
            self.c = self.c.wrapping_sub(1);
            if self.c != 0 {
                self.pc = address
            }
        }
        Instruction::LoopOffset(offset) => {
            self.c = self.c.wrapping_sub(1);
            if self.c != 0 {
                self.pc = self.b.wrapping_add(offset)
            }
        }
        Instruction::LoopRelative(offset) => {
            self.c = self.c.wrapping_sub(1);
            if self.c != 0 {
                self.pc = self.pc.wrapping_add(offset)
            }
        }
        Instruction::Call(address) => {
            self.sp = self.sp.wrapping_sub(2);
            self.memory[self.sp as usize] = self.pc as u8;
            self.memory[self.sp.wrapping_add(1) as usize] = (self.pc >> 8) as u8;
            self.pc = address;
        }
        Instruction::CallOffset(offset) => {
            self.sp = self.sp.wrapping_sub(2);
            self.memory[self.sp as usize] = self.pc as u8;
            self.memory[self.sp.wrapping_add(1) as usize] = (self.pc >> 8) as u8;
            self.pc = self.b.wrapping_add(offset)
        }
        Instruction::CallRelative(offset) => {
            self.sp = self.sp.wrapping_sub(2);
            self.memory[self.sp as usize] = self.pc as u8;
            self.memory[self.sp.wrapping_add(1) as usize] = (self.pc >> 8) as u8;
            self.pc = self.pc.wrapping_add(offset)
        }
        Instruction::Push => {
            self.sp = self.sp.wrapping_sub(2);
            self.memory[self.sp as usize] = self.a as u8;
            self.memory[self.sp.wrapping_add(1) as usize] = (self.a >> 8) as u8
        }
        Instruction::PushPC => {
            self.sp = self.sp.wrapping_sub(2);
            self.memory[self.sp as usize] = self.pc as u8;
            self.memory[self.sp.wrapping_add(1) as usize] = (self.pc >> 8) as u8
        }
        Instruction::PushFlags => {
            self.sp = self.sp.wrapping_sub(2);
            self.memory[self.sp as usize] = self.flags as u8;
            self.memory[self.sp.wrapping_add(1) as usize] = (self.flags >> 8) as u8
        }
        Instruction::Pop => {
            self.a = u16::from_le_bytes([
                self.memory[self.sp as usize],
                self.memory[self.sp.wrapping_add(1) as usize],
            ]);
            self.sp = self.sp.wrapping_add(2)
        }
        Instruction::Return => {
            self.pc = u16::from_le_bytes([
                self.memory[self.sp as usize],
                self.memory[self.sp.wrapping_add(1) as usize],
            ]);
            self.sp = self.sp.wrapping_add(2)
        }
        Instruction::PopFlags => {
            self.flags = u16::from_le_bytes([
                self.memory[self.sp as usize],
                self.memory[self.sp.wrapping_add(1) as usize],
            ]);
            self.sp = self.sp.wrapping_add(2)
        }
        Instruction::Input => {
            self.a = match stdin().read_array::<1>() {
                Ok(arr) => arr[0] as u16,
                Err(_) => u16::MAX,
            }
        }
        Instruction::Output => {
            print!("{}", self.a as u8 as char)
        }
        Instruction::SetInterrupt(address) => {
            self.memory[0xFFFE] = address as u8;
            self.memory[0xFFFF] = (address >> 8) as u8
        }
        Instruction::Clear(flag) => self.flags &= !(1 << flag),
        Instruction::Set(flag) => self.flags |= 1 << flag,
    }
}
}