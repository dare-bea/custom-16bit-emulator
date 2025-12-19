use crate::flag;
use crate::isa::Instruction::{self, *};
use crate::register::Register;

use super::{CPU, Emulator, Memory};

impl CPU {
    fn register(&self, reg: Register) -> u16 {
        match reg {
            Register::A => self.a,
            Register::B => self.b,
            Register::C => self.c,
            Register::D => self.d,
        }
    }

    fn mut_register(&mut self, reg: Register) -> &mut u16 {
        match reg {
            Register::A => &mut self.a,
            Register::B => &mut self.b,
            Register::C => &mut self.c,
            Register::D => &mut self.d,
        }
    }

    pub fn execute_instruction(&mut self, instruction: &Instruction, emulator: &mut Emulator) {
        match *instruction {
            LoadImmediate(reg, value) => {
                *self.mut_register(reg) = value;
            }
            LoadAddressAbsolute(addr) => {
                self.a = emulator.memory.read_word(addr.into());
            }
            LoadAddressStackOffset(offset) => {
                self.a = emulator.memory.read_word(self.sp.wrapping_add(offset as u16).into());
            }
            LoadAddressIndirect(addr, reg) => {
                self.a = emulator.memory.read_word(addr.wrapping_add(self.register(reg)).into());
            }
            StoreAddressAbsolute(addr) => {
                emulator.memory.write(addr.into(), self.a as u8);
            }
            StoreAddressStackOffset(offset) => {
                emulator.memory.write(self.sp.wrapping_add(offset as u16).into(), self.a as u8);
            }
            StoreAddressIndirect(addr, reg) => {
                emulator.memory.write(addr.wrapping_add(self.register(reg)).into(), self.a as u8);
            }
            StoreWordAbsolute(addr) => {
                emulator.memory.write_word(addr.into(), self.a);
            }
            StoreWordStackOffset(offset) => {
                emulator.memory.write_word(self.sp.wrapping_add(offset as u16).into(), self.a);
            }
            StoreWordIndirect(addr, reg) => {
                emulator.memory.write_word(addr.wrapping_add(self.register(reg)).into(), self.a);
            }
            MoveRegister(src, dst) => {
                *self.mut_register(dst) = self.register(src);
            }
            MoveRegisterToSP(reg) => {
                self.sp = self.register(reg);
            }
            MoveSPToRegister(reg) => {
                *self.mut_register(reg) = self.sp;
            }
            And(reg) => {
                self.a &= self.register(reg);
                flag::set_flag(&mut self.flags, flag::ZERO, self.a == 0);
                flag::set_flag(&mut self.flags, flag::SIGN, self.a & 0x8000 != 0);
            }
            Or(reg) => {
                self.a |= self.register(reg);
                flag::set_flag(&mut self.flags, flag::ZERO, self.a == 0);
                flag::set_flag(&mut self.flags, flag::SIGN, self.a & 0x8000 != 0);
            }
            Xor(reg) => {
                self.a ^= self.register(reg);
                flag::set_flag(&mut self.flags, flag::ZERO, self.a == 0);
                flag::set_flag(&mut self.flags, flag::SIGN, self.a & 0x8000 != 0);
            }
            ShiftLeft(reg) => {
                let (result, carry) = self.a.overflowing_shl(self.register(reg) as u32);
                self.a = result;
                flag::set_flag(&mut self.flags, flag::CARRY, carry);
                flag::set_flag(&mut self.flags, flag::ZERO, result == 0);
                flag::set_flag(&mut self.flags, flag::SIGN, result & 0x8000 != 0);
            }
            ShiftRight(reg) => {
                let (result, carry) = self.a.overflowing_shr(self.register(reg) as u32);
                self.a = result;
                flag::set_flag(&mut self.flags, flag::CARRY, carry);
                flag::set_flag(&mut self.flags, flag::ZERO, result == 0);
                flag::set_flag(&mut self.flags, flag::SIGN, result & 0x8000 != 0);
            }
            Add(reg) => {
                let (result, carry) = self.a.overflowing_add(self.register(reg));
                let (_, overflow) = (self.a as i16).overflowing_add(self.register(reg) as i16);
                self.a = result;
                flag::set_flag(&mut self.flags, flag::CARRY, carry);
                flag::set_flag(&mut self.flags, flag::OVERFLOW, overflow);
                flag::set_flag(&mut self.flags, flag::ZERO, result == 0);
                flag::set_flag(&mut self.flags, flag::SIGN, result & 0x8000 != 0);
            }
            Subtract(reg) => {
                let (result, carry) = self.a.overflowing_sub(self.register(reg));
                let (_, overflow) = (self.a as i16).overflowing_sub(self.register(reg) as i16);
                self.a = result;
                flag::set_flag(&mut self.flags, flag::CARRY, carry);
                flag::set_flag(&mut self.flags, flag::OVERFLOW, overflow);
                flag::set_flag(&mut self.flags, flag::ZERO, result == 0);
                flag::set_flag(&mut self.flags, flag::SIGN, result & 0x8000 != 0);
            }
            RotateLeft(reg) => {
                self.a = self.a.rotate_left(self.register(reg) as u32);
                flag::set_flag(&mut self.flags, flag::ZERO, self.a == 0);
                flag::set_flag(&mut self.flags, flag::SIGN, self.a & 0x8000 != 0);
            }
            RotateRight(reg) => {
                self.a = self.a.rotate_right(self.register(reg) as u32);
                flag::set_flag(&mut self.flags, flag::ZERO, self.a == 0);
                flag::set_flag(&mut self.flags, flag::SIGN, self.a & 0x8000 != 0);
            }
            AddWithCarry(reg) => {
                let carry_before = flag::get_flag(self.flags, flag::CARRY);
                let (result, carry) = self.a.carrying_add(self.register(reg) as u16, carry_before);
                let (_, overflow) = (self.a as i16).carrying_add((self.register(reg) + carry as u16) as i16, carry_before);
                self.a = result;
                flag::set_flag(&mut self.flags, flag::CARRY, carry);
                flag::set_flag(&mut self.flags, flag::OVERFLOW, overflow);
                flag::set_flag(&mut self.flags, flag::ZERO, result == 0);
                flag::set_flag(&mut self.flags, flag::SIGN, result & 0x8000 != 0);
            }
            SubtractWithBorrow(reg) => {
                let carry_before = flag::get_flag(self.flags, flag::CARRY);
                let (result, carry) = self.a.borrowing_sub(self.register(reg) as u16, carry_before);
                let (_, overflow) = (self.a as i16).borrowing_sub((self.register(reg) - carry as u16) as i16, carry_before);
                self.a = result;
                flag::set_flag(&mut self.flags, flag::CARRY, carry);
                flag::set_flag(&mut self.flags, flag::OVERFLOW, overflow);
                flag::set_flag(&mut self.flags, flag::ZERO, result == 0);
                flag::set_flag(&mut self.flags, flag::SIGN, result & 0x8000 != 0);
            }
            Negate(reg) => {
                let (result, carry) = self.register(reg).overflowing_neg();
                let (_, overflow) = (self.register(reg) as i16).overflowing_neg();
                self.a = result;
                flag::set_flag(&mut self.flags, flag::CARRY, carry);
                flag::set_flag(&mut self.flags, flag::OVERFLOW, overflow);
                flag::set_flag(&mut self.flags, flag::ZERO, result == 0);
                flag::set_flag(&mut self.flags, flag::SIGN, result & 0x8000 != 0);
            }
            Not(reg) => {
                self.a = !self.register(reg);
                flag::set_flag(&mut self.flags, flag::ZERO, self.a == 0);
                flag::set_flag(&mut self.flags, flag::SIGN, self.a & 0x8000 != 0);
            }
            Increment(reg) => {
                let (result, carry) = self.register(reg).overflowing_add(1);
                let (_, overflow) = (self.register(reg) as i16).overflowing_add(1);
                *self.mut_register(reg) = result;
                flag::set_flag(&mut self.flags, flag::CARRY, carry);
                flag::set_flag(&mut self.flags, flag::OVERFLOW, overflow);
                flag::set_flag(&mut self.flags, flag::ZERO, result == 0);
                flag::set_flag(&mut self.flags, flag::SIGN, result & 0x8000 != 0);
            }
            Decrement(reg) => {
                let (result, carry) = self.register(reg).overflowing_sub(1);
                let (_, overflow) = (self.register(reg) as i16).overflowing_sub(1);
                *self.mut_register(reg) = result;
                flag::set_flag(&mut self.flags, flag::CARRY, carry);
                flag::set_flag(&mut self.flags, flag::OVERFLOW, overflow);
                flag::set_flag(&mut self.flags, flag::ZERO, result == 0);
                flag::set_flag(&mut self.flags, flag::SIGN, result & 0x8000 != 0);
            }
        }
    }
}