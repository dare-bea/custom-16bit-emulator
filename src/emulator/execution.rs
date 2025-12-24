use super::{Emulator, Memory};
use crate::flag;
use crate::isa::Instruction::{self, *};

impl Emulator {
    pub fn next_cpu_instruction(&self) -> (Instruction, u32) {
        Instruction::try_from_iter(self.memory.iter(self.cpu.pc.into()))
            .expect("Failed to decode instruction")
    }

    pub fn advance_cpu(&mut self) {
        let (instruction, byte_length) = self.next_cpu_instruction();
        self.cpu.pc = self.cpu.pc.wrapping_add(byte_length as u16);
        self.execute_cpu_instruction(&instruction);
    }

    pub fn handle_interrupt(&mut self) {
        if self.cpu.ir_flags == 0 {
            return;
        }
        let irq = self.cpu.ir_flags.trailing_zeros() as u8;
        self.cpu.ir_flags &= !(1 << irq);
        self.cpu.sp = self.cpu.sp.wrapping_sub(2);
        self.memory.write_word(self.cpu.sp.into(), self.cpu.pc);
        self.cpu.sp = self.cpu.sp.wrapping_sub(1);
        self.memory.write(self.cpu.sp.into(), self.cpu.flags);
    }

    pub fn handle_return_from_interrupt(&mut self) {
        self.cpu.flags = self.memory.read(self.cpu.sp.into());
        self.cpu.sp = self.cpu.sp.wrapping_add(1);
        self.cpu.pc = self.memory.read_word(self.cpu.sp.into());
        self.cpu.sp = self.cpu.sp.wrapping_add(2);
        // Check if there are more interrupts to handle, and handle them.
        self.handle_interrupt();
    }

    pub fn execute_cpu_instruction(&mut self, instruction: &Instruction) {
        match *instruction {
            LoadImmediate(reg, value) => {
                *self.cpu.mut_register(reg) = value;
            }
            LoadAddressAbsolute(addr) => {
                self.cpu.a = self.memory.read(addr.into()).into();
            }
            LoadAddressStackOffset(offset) => {
                self.cpu.a = self
                    .memory
                    .read(self.cpu.sp.wrapping_add(offset as u16).into())
                    .into();
            }
            LoadAddressIndirect(addr, reg) => {
                self.cpu.a = self
                    .memory
                    .read(addr.wrapping_add(self.cpu.register(reg)).into())
                    .into();
            }
            LoadWordAbsolute(addr) => {
                self.cpu.a = self.memory.read_word(addr.into());
            }
            LoadWordStackOffset(offset) => {
                self.cpu.a = self
                    .memory
                    .read_word(self.cpu.sp.wrapping_add(offset as u16).into());
            }
            LoadWordIndirect(addr, reg) => {
                self.cpu.a = self
                    .memory
                    .read_word(addr.wrapping_add(self.cpu.register(reg)).into());
            }
            StoreAddressAbsolute(addr) => {
                self.memory.write(addr.into(), self.cpu.a as u8);
            }
            StoreAddressStackOffset(offset) => {
                self.memory.write(
                    self.cpu.sp.wrapping_add(offset as u16).into(),
                    self.cpu.a as u8,
                );
            }
            StoreAddressIndirect(addr, reg) => {
                self.memory.write(
                    addr.wrapping_add(self.cpu.register(reg)).into(),
                    self.cpu.a as u8,
                );
            }
            StoreWordAbsolute(addr) => {
                self.memory.write_word(addr.into(), self.cpu.a);
            }
            StoreWordStackOffset(offset) => {
                self.memory
                    .write_word(self.cpu.sp.wrapping_add(offset as u16).into(), self.cpu.a);
            }
            StoreWordIndirect(addr, reg) => {
                self.memory
                    .write_word(addr.wrapping_add(self.cpu.register(reg)).into(), self.cpu.a);
            }
            MoveRegister(src, dst) => {
                *self.cpu.mut_register(dst) = self.cpu.register(src);
            }
            MoveRegisterToSP(reg) => {
                self.cpu.sp = self.cpu.register(reg);
            }
            MoveSPToRegister(reg) => {
                *self.cpu.mut_register(reg) = self.cpu.sp;
            }
            And(reg) => {
                self.cpu.a &= self.cpu.register(reg);
                flag::set_flag(&mut self.cpu.flags, flag::ZERO, self.cpu.a == 0);
                flag::set_flag(&mut self.cpu.flags, flag::SIGN, self.cpu.a & 0x8000 != 0);
            }
            Or(reg) => {
                self.cpu.a |= self.cpu.register(reg);
                flag::set_flag(&mut self.cpu.flags, flag::ZERO, self.cpu.a == 0);
                flag::set_flag(&mut self.cpu.flags, flag::SIGN, self.cpu.a & 0x8000 != 0);
            }
            Xor(reg) => {
                self.cpu.a ^= self.cpu.register(reg);
                flag::set_flag(&mut self.cpu.flags, flag::ZERO, self.cpu.a == 0);
                flag::set_flag(&mut self.cpu.flags, flag::SIGN, self.cpu.a & 0x8000 != 0);
            }
            ShiftLeft(reg) => {
                let (result, carry) = self.cpu.a.overflowing_shl(self.cpu.register(reg) as u32);
                self.cpu.a = result;
                flag::set_flag(&mut self.cpu.flags, flag::CARRY, carry);
                flag::set_flag(&mut self.cpu.flags, flag::ZERO, result == 0);
                flag::set_flag(&mut self.cpu.flags, flag::SIGN, result & 0x8000 != 0);
            }
            ShiftRight(reg) => {
                let (result, carry) = self.cpu.a.overflowing_shr(self.cpu.register(reg) as u32);
                self.cpu.a = result;
                flag::set_flag(&mut self.cpu.flags, flag::CARRY, carry);
                flag::set_flag(&mut self.cpu.flags, flag::ZERO, result == 0);
                flag::set_flag(&mut self.cpu.flags, flag::SIGN, result & 0x8000 != 0);
            }
            Add(reg) => {
                let (result, carry) = self.cpu.a.overflowing_add(self.cpu.register(reg));
                let (_, overflow) =
                    (self.cpu.a as i16).overflowing_add(self.cpu.register(reg) as i16);
                self.cpu.a = result;
                flag::set_flag(&mut self.cpu.flags, flag::CARRY, carry);
                flag::set_flag(&mut self.cpu.flags, flag::OVERFLOW, overflow);
                flag::set_flag(&mut self.cpu.flags, flag::ZERO, result == 0);
                flag::set_flag(&mut self.cpu.flags, flag::SIGN, result & 0x8000 != 0);
            }
            Subtract(reg) => {
                let (result, carry) = self.cpu.a.overflowing_sub(self.cpu.register(reg));
                let (_, overflow) =
                    (self.cpu.a as i16).overflowing_sub(self.cpu.register(reg) as i16);
                self.cpu.a = result;
                flag::set_flag(&mut self.cpu.flags, flag::CARRY, carry);
                flag::set_flag(&mut self.cpu.flags, flag::OVERFLOW, overflow);
                flag::set_flag(&mut self.cpu.flags, flag::ZERO, result == 0);
                flag::set_flag(&mut self.cpu.flags, flag::SIGN, result & 0x8000 != 0);
            }
            RotateLeft(reg) => {
                self.cpu.a = self.cpu.a.rotate_left(self.cpu.register(reg) as u32);
                flag::set_flag(&mut self.cpu.flags, flag::ZERO, self.cpu.a == 0);
                flag::set_flag(&mut self.cpu.flags, flag::SIGN, self.cpu.a & 0x8000 != 0);
            }
            RotateRight(reg) => {
                self.cpu.a = self.cpu.a.rotate_right(self.cpu.register(reg) as u32);
                flag::set_flag(&mut self.cpu.flags, flag::ZERO, self.cpu.a == 0);
                flag::set_flag(&mut self.cpu.flags, flag::SIGN, self.cpu.a & 0x8000 != 0);
            }
            AddWithCarry(reg) => {
                let carry_before = flag::get_flag(self.cpu.flags, flag::CARRY);
                let (result, carry) = self
                    .cpu
                    .a
                    .carrying_add(self.cpu.register(reg), carry_before);
                let (_, overflow) = (self.cpu.a as i16)
                    .carrying_add((self.cpu.register(reg) + carry as u16) as i16, carry_before);
                self.cpu.a = result;
                flag::set_flag(&mut self.cpu.flags, flag::CARRY, carry);
                flag::set_flag(&mut self.cpu.flags, flag::OVERFLOW, overflow);
                flag::set_flag(&mut self.cpu.flags, flag::ZERO, result == 0);
                flag::set_flag(&mut self.cpu.flags, flag::SIGN, result & 0x8000 != 0);
            }
            SubtractWithBorrow(reg) => {
                let carry_before = flag::get_flag(self.cpu.flags, flag::CARRY);
                let (result, carry) = self
                    .cpu
                    .a
                    .borrowing_sub(self.cpu.register(reg), carry_before);
                let (_, overflow) = (self.cpu.a as i16)
                    .borrowing_sub((self.cpu.register(reg) - carry as u16) as i16, carry_before);
                self.cpu.a = result;
                flag::set_flag(&mut self.cpu.flags, flag::CARRY, carry);
                flag::set_flag(&mut self.cpu.flags, flag::OVERFLOW, overflow);
                flag::set_flag(&mut self.cpu.flags, flag::ZERO, result == 0);
                flag::set_flag(&mut self.cpu.flags, flag::SIGN, result & 0x8000 != 0);
            }
            Negate(reg) => {
                let (result, carry) = self.cpu.register(reg).overflowing_neg();
                let (_, overflow) = (self.cpu.register(reg) as i16).overflowing_neg();
                self.cpu.a = result;
                flag::set_flag(&mut self.cpu.flags, flag::CARRY, carry);
                flag::set_flag(&mut self.cpu.flags, flag::OVERFLOW, overflow);
                flag::set_flag(&mut self.cpu.flags, flag::ZERO, result == 0);
                flag::set_flag(&mut self.cpu.flags, flag::SIGN, result & 0x8000 != 0);
            }
            Not(reg) => {
                self.cpu.a = !self.cpu.register(reg);
                flag::set_flag(&mut self.cpu.flags, flag::ZERO, self.cpu.a == 0);
                flag::set_flag(&mut self.cpu.flags, flag::SIGN, self.cpu.a & 0x8000 != 0);
            }
            Increment(reg) => {
                let (result, carry) = self.cpu.register(reg).overflowing_add(1);
                let (_, overflow) = (self.cpu.register(reg) as i16).overflowing_add(1);
                *self.cpu.mut_register(reg) = result;
                flag::set_flag(&mut self.cpu.flags, flag::CARRY, carry);
                flag::set_flag(&mut self.cpu.flags, flag::OVERFLOW, overflow);
                flag::set_flag(&mut self.cpu.flags, flag::ZERO, result == 0);
                flag::set_flag(&mut self.cpu.flags, flag::SIGN, result & 0x8000 != 0);
            }
            Decrement(reg) => {
                let (result, carry) = self.cpu.register(reg).overflowing_sub(1);
                let (_, overflow) = (self.cpu.register(reg) as i16).overflowing_sub(1);
                *self.cpu.mut_register(reg) = result;
                flag::set_flag(&mut self.cpu.flags, flag::CARRY, carry);
                flag::set_flag(&mut self.cpu.flags, flag::OVERFLOW, overflow);
                flag::set_flag(&mut self.cpu.flags, flag::ZERO, result == 0);
                flag::set_flag(&mut self.cpu.flags, flag::SIGN, result & 0x8000 != 0);
            }
            Compare(reg) => {
                let (result, carry) = self.cpu.a.overflowing_sub(self.cpu.register(reg));
                let (_, overflow) =
                    (self.cpu.a as i16).overflowing_sub(self.cpu.register(reg) as i16);
                flag::set_flag(&mut self.cpu.flags, flag::CARRY, carry);
                flag::set_flag(&mut self.cpu.flags, flag::OVERFLOW, overflow);
                flag::set_flag(&mut self.cpu.flags, flag::ZERO, result == 0);
                flag::set_flag(&mut self.cpu.flags, flag::SIGN, result & 0x8000 != 0);
            }
            Test(reg) => {
                let result = self.cpu.a & self.cpu.register(reg);
                flag::set_flag(&mut self.cpu.flags, flag::ZERO, result == 0);
                flag::set_flag(&mut self.cpu.flags, flag::SIGN, result & 0x8000 != 0);
            }
            CompareImmediate(reg, imm) => {
                let (result, carry) = self.cpu.register(reg).overflowing_sub(imm);
                let (_, overflow) = (self.cpu.register(reg) as i16).overflowing_sub(imm as i16);
                flag::set_flag(&mut self.cpu.flags, flag::CARRY, carry);
                flag::set_flag(&mut self.cpu.flags, flag::OVERFLOW, overflow);
                flag::set_flag(&mut self.cpu.flags, flag::ZERO, result == 0);
                flag::set_flag(&mut self.cpu.flags, flag::SIGN, result & 0x8000 != 0);
            }
            TestImmediate(reg, imm) => {
                let result = self.cpu.register(reg) & imm;
                flag::set_flag(&mut self.cpu.flags, flag::ZERO, result == 0);
                flag::set_flag(&mut self.cpu.flags, flag::SIGN, result & 0x8000 != 0);
            }
            JumpAbsolute(addr) => {
                self.cpu.pc = addr;
            }
            JumpNear(offset) => {
                self.cpu.pc = self.cpu.pc.wrapping_add(offset as u16);
            }
            JumpStackOffset(offset) => {
                self.cpu.pc = self.cpu.sp.wrapping_add(offset as u16);
            }
            Call(addr) => {
                self.cpu.sp = self.cpu.sp.wrapping_sub(2);
                self.memory.write_word(self.cpu.sp.into(), self.cpu.pc);
                self.cpu.pc = addr;
            }
            JumpIndirect(addr, reg) => {
                self.cpu.pc = self
                    .memory
                    .read_word(addr.wrapping_add(self.cpu.register(reg)).into());
            }
            JumpIf(cond, addr) => {
                if flag::get_flag(self.cpu.flags, cond) {
                    self.cpu.pc = addr;
                }
            }
            PushPC => {
                self.cpu.sp = self.cpu.sp.wrapping_sub(2);
                self.memory.write_word(self.cpu.sp.into(), self.cpu.pc);
            }
            PopPC => {
                self.cpu.pc = self.memory.read_word(self.cpu.sp.into());
                self.cpu.sp = self.cpu.sp.wrapping_add(2);
            }
            PushFlags => {
                self.cpu.sp = self.cpu.sp.wrapping_sub(1);
                self.memory.write(self.cpu.sp.into(), self.cpu.flags);
            }
            PopFlags => {
                self.cpu.flags = self.memory.read(self.cpu.sp.into());
                self.cpu.sp = self.cpu.sp.wrapping_add(1);
            }
            PushRegister(reg) => {
                self.cpu.sp = self.cpu.sp.wrapping_sub(2);
                self.memory
                    .write_word(self.cpu.sp.into(), self.cpu.register(reg));
            }
            PopRegister(reg) => {
                *self.cpu.mut_register(reg) = self.memory.read_word(self.cpu.sp.into());
                self.cpu.sp = self.cpu.sp.wrapping_add(2);
            }
            ClearInterruptRequest(irq) => {
                self.cpu.ir_flags &= !(1 << irq);
            }
            SetInterruptRequest(irq) => {
                self.cpu.ir_flags |= 1 << irq;
            }
            WaitForInterrupt => {
                if self.cpu.ir_flags == 0 {
                    self.cpu.pc = self.cpu.pc.wrapping_sub(1);
                } else {
                    self.handle_interrupt();
                }
            }
            ReturnFromInterrupt => {
                self.handle_return_from_interrupt();
            }
            ClearFlags(flags) => {
                self.cpu.flags &= !flags;
            }
            SetFlags(flags) => {
                self.cpu.flags |= flags;
            }
        }
    }

    pub fn advance(&mut self) {
        self.advance_cpu();
    }
}
