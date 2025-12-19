use crate::emulator::Emulator;
use crate::memory::Memory;
use crate::isa::Instruction::{self, *};
use crate::register::Register;

impl<M: Memory> Emulator<M> {
    pub fn write_register(&mut self, reg: Register, value: u16) {
        *self.mut_register(reg) = value;
    }

    pub fn read_register(&self, reg: Register) -> u16 {
        self.register(reg)
    }

    pub fn port_in(&mut self, port: u8) -> u8 {
        todo!()
    }

    pub fn port_out(&mut self, port: u8, value: u8) {
        todo!()
    }

    pub fn read_memory_byte(&self, address: usize) -> u8 {
        self.memory.read_byte(address)
    }

    pub fn read_memory(&self, address: usize) -> u16 {
        let low = self.memory.read_byte(address) as u16;
        let high = self.memory.read_byte(address + 1) as u16;
        u16::from_le_bytes([low as u8, high as u8])
    }

    pub fn execute(&mut self, instruction: Instruction) {
        match instruction {
            LoadImmediate(reg, value) => self.write_register(reg, value),
            LoadAddressAbsolute(address) => {
                let value = self.read_memory(address) as u16;
                self.write_register(Register::A, value);
            }
        }
    }
}