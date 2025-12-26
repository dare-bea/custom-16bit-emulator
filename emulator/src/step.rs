use super::Emulator;
use std::io;
use utils::{condition::ConditionCode, register::Register};

pub enum EmulationError {
    IOError(io::Error),
    RegisterIndexError(()),
    InvalidOpcode(u8),
    InvalidCondition(()),
}

impl From<io::Error> for EmulationError {
    fn from(value: io::Error) -> Self {
        Self::IOError(value)
    }
}

impl Emulator {
    pub fn step(&mut self) -> Result<(), EmulationError> {
        match self.next_byte()? {
            0x00 => {
                // LD addr
                let addr = self.next_word()?;
                self.cpu.a = self.memory.read_byte(addr)? as u16;
            }
            0x01 => {
                // LD addr, SP
                let addr = self.next_word()?.wrapping_add(self.cpu.sp);
                self.cpu.a = self.memory.read_byte(addr)? as u16;
            }
            0x03 => {
                // LDS dst, src
                let (dst, src) = Register::pair_from(&self.next_byte()?).map_err(EmulationError::RegisterIndexError)?;
                *self.cpu.register_mut(dst) = (self.memory.read_byte(self.cpu.register(src))? as u16).wrapping_add(self.cpu.sp);
            }
            0x04 => {
                // LDW addr
                let addr = self.next_word()?;
                self.cpu.a = self.memory.read_word(addr)?;
            }
            0x05 => {
                // LDW addr, SP
                let addr = self.next_word()?.wrapping_add(self.cpu.sp);
                self.cpu.a = self.memory.read_word(addr)?;
            }
            0x07 => {
                // LDSW dst, src
                let (dst, src) = Register::pair_from(&self.next_byte()?).map_err(EmulationError::RegisterIndexError)?;
                *self.cpu.register_mut(dst) = (self.memory.read_word(self.cpu.register(src))?).wrapping_add(self.cpu.sp);
            }
            op @ 0x08..=0x0B => {
                // LD addr, reg
                let addr: u16 = self.next_word()?.wrapping_add(self.cpu.register(Register::try_from(op & 0x3).unwrap()));
                self.cpu.a = self.memory.read_byte(addr)? as u16;
            }
            op @ 0x0C..=0x0F => {
                // LDW addr, reg
                let addr: u16 = self.next_word()?.wrapping_add(self.cpu.register(Register::try_from(op & 0x3).unwrap()));
                self.cpu.a = self.memory.read_word(addr)? as u16;
            }
            0x10 => {
                // ST addr
                let addr = self.next_word()?;
                self.memory.write_byte(addr, self.cpu.a as u8)?;
            }
            0x11 => {
                // ST addr, sp
                let addr = self.next_word()?.wrapping_add(self.cpu.sp);
                self.memory.write_byte(addr, self.cpu.a as u8)?;
            }
            0x13 => {
                // STS dst, src
                let (dst, src) = Register::pair_from(&self.next_byte()?).map_err(EmulationError::RegisterIndexError)?;
                self.memory.write_byte(self.cpu.register(dst), self.cpu.register(src) as u8)?;
            }
            0x14 => {
                // STW addr
                let addr = self.next_word()?;
                self.memory.write_word(addr, self.cpu.a)?;
            }
            0x15 => {
                // STW addr, sp
                let addr = self.next_word()?.wrapping_add(self.cpu.sp);
                self.memory.write_word(addr, self.cpu.a)?;
            }
            0x17 => {
                // STSW dst, src
                let (dst, src) = Register::pair_from(&self.next_byte()?).map_err(EmulationError::RegisterIndexError)?;
                self.memory.write_word(self.cpu.register(dst), self.cpu.register(src))?;
            }
            op @ 0x18..=0x1B => {
                // ST addr, reg
                let addr: u16 = self.next_word()?.wrapping_add(self.cpu.register(Register::try_from(op & 0x3).unwrap()));
                self.memory.write_byte(addr, self.cpu.a as u8)?;
            }
            op @ 0x1C..=0x1F => {
                // STW addr, reg
                let addr: u16 = self.next_word()?.wrapping_add(self.cpu.register(Register::try_from(op & 0x3).unwrap()));
                self.memory.write_word(addr, self.cpu.a)?;
            }
            op @ 0x20..=0x23 => {
                // LDI addr, #imm8
                *self.cpu.register_mut(Register::try_from(op & 0x3).unwrap()) = self.next_byte()? as u16;
            }
            op @ 0x24..=0x27 => {
                // LDI addr, #imm16
                *self.cpu.register_mut(Register::try_from(op & 0x3).unwrap()) = self.next_word()?;
            }
            op @ (0x28 | 0x30 | 0x38) => {
                // JMP rel, PC
                match op & !0x07 {
                    0x28 => (),
                    0x30 if
                        !ConditionCode::try_from(self.next_byte()?).map_err(EmulationError::InvalidCondition)?.meets(self.cpu.flags) => return Ok(()),
                    0x38 => {self.push(self.cpu.pc)?;}
                    _ => unreachable!()
                };
                self.cpu.pc = self.cpu.pc.wrapping_add_signed(self.next_byte()? as i8 as i16);
            }
            op @ (0x29 | 0x31 | 0x39) => {
                // JMP addr
                match op & !0x07 {
                    0x28 => (),
                    0x30 if
                        !ConditionCode::try_from(self.next_byte()?).map_err(EmulationError::InvalidCondition)?.meets(self.cpu.flags) => return Ok(()),
                    0x38 => {self.push(self.cpu.pc)?;}
                    _ => unreachable!()
                };
                self.cpu.pc = self.next_word()?;
            }
            op @ (0x2A | 0x32 | 0x3A) => {
                // JMP addr, SP
                match op & !0x07 {
                    0x28 => (),
                    0x30 if
                        !ConditionCode::try_from(self.next_byte()?).map_err(EmulationError::InvalidCondition)?.meets(self.cpu.flags) => return Ok(()),
                    0x38 => {self.push(self.cpu.pc)?;}
                    _ => unreachable!()
                };
                self.cpu.pc = self.next_word()?.wrapping_add(self.cpu.sp);
            }
            op @ (0x2B | 0x33 | 0x3B) => {
                // MOV dst, src
                let (dst, src) = Register::pair_from(&self.next_byte()?).map_err(EmulationError::RegisterIndexError)?;
                match op & !0x07 {
                    0x28 => (),
                    0x30 if
                        !ConditionCode::try_from(self.next_byte()?).map_err(EmulationError::InvalidCondition)?.meets(self.cpu.flags) => return Ok(()),
                    0x38 => {self.push(self.cpu.register(dst))?;}
                    _ => unreachable!()
                };
                *self.cpu.register_mut(dst) = self.cpu.register(src)
            }
            op @ (0x2C..=0x2F | 0x34..=0x37 | 0x3C..=0x3F) => {
                // JMP addr, reg
                match op & !0x07 {
                    0x28 => (),
                    0x30 if
                        !ConditionCode::try_from(self.next_byte()?).map_err(EmulationError::InvalidCondition)?.meets(self.cpu.flags) => return Ok(()),
                    0x38 => {self.push(self.cpu.pc)?;}
                    _ => unreachable!()
                };
                self.cpu.pc = self.next_word()?.wrapping_add(self.cpu.register(Register::try_from(op & 0x3).unwrap()));
            }
            op => return Err(EmulationError::InvalidOpcode(op)),
        };
        Ok(())
    }
}
