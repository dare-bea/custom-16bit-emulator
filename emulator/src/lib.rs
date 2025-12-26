use cpu::Cpu;
use memory::{Mmu, Ram, SimpleRom};
use std::{fmt::Debug, io};
use utils::flag::Flag;

pub mod cpu;
pub mod memory;
pub mod step;

#[derive(Debug)]
pub struct Emulator {
    pub cpu: Cpu,
    pub memory: Mmu,
}

impl Emulator {
    pub fn new() -> io::Result<Self> {
        let mut emu = Self {
            cpu: Cpu::default(),
            memory: Mmu::new(Ram::new([0; _]), Box::new(SimpleRom::new([0; _])))?,
        };
        emu.reset()?;
        Ok(emu)
    }

    pub fn reset(&mut self) -> io::Result<()> {
        self.cpu = Cpu::new();
        self.cpu.pc = self.memory.read_word(0xFFFE)?;
        if self.cpu.pc == 0 {
            self.cpu.flags |= Flag::Halt.to_bitmask(); // TODO: Add Display
        }
        Ok(())
    }

    pub fn next_byte(&mut self) -> io::Result<u8> {
        let value = self.memory.read_byte(self.cpu.pc)?;
        self.cpu.pc = self.cpu.pc.wrapping_add(1);
        Ok(value)
    }

    pub fn next_word(&mut self) -> io::Result<u16> {
        let value = self.memory.read_word(self.cpu.pc)?;
        self.cpu.pc = self.cpu.pc.wrapping_add(2);
        Ok(value)
    }

    pub fn push(&mut self, value: u16) -> io::Result<()> {
        self.cpu.sp = self.cpu.sp.wrapping_sub(2);
        self.memory.write_word(self.cpu.sp, value)
    }

    pub fn pop(&mut self) -> io::Result<u16> {
        let result = self.memory.read_word(self.cpu.sp);
        self.cpu.sp = self.cpu.sp.wrapping_add(2);
        result
    }
}
