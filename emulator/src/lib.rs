use cpu::Cpu;
use memory::{Mmu, Ram, SimpleRom};
use std::{fmt::Debug, io::Result};
use utils::flag::Flag;
use port::PortHandler;

pub mod cpu;
pub mod memory;
pub mod step;
pub mod port;

#[derive(Debug)]
pub struct Emulator {
    pub cpu: Cpu,
    pub memory: Mmu,
    pub ports: PortHandler,
}

impl Emulator {
    pub fn new() -> Result<Self> {
        let mut emu = Self {
            cpu: Cpu::default(),
            memory: Mmu::new(Ram::new([0; _]), Box::new(SimpleRom::new([0; _])))?,
            ports: PortHandler::new(),
        };
        emu.reset()?;
        Ok(emu)
    }

    pub fn reset(&mut self) -> Result<()> {
        self.cpu = Cpu::new();
        self.cpu.pc = self.memory.read_word(0xFFFE)?;
        if self.cpu.pc == 0 {
            self.cpu.flags |= Flag::Halt.to_bitmask(); // TODO: Add Display
        }
        Ok(())
    }

    pub fn next_byte(&mut self) -> Result<u8> {
        let value = self.memory.read_byte(self.cpu.pc)?;
        self.cpu.pc = self.cpu.pc.wrapping_add(1);
        Ok(value)
    }

    pub fn next_word(&mut self) -> Result<u16> {
        let value = self.memory.read_word(self.cpu.pc)?;
        self.cpu.pc = self.cpu.pc.wrapping_add(2);
        Ok(value)
    }

    pub fn push(&mut self, value: u16) -> Result<()> {
        self.cpu.sp = self.cpu.sp.wrapping_sub(2);
        self.memory.write_word(self.cpu.sp, value)
    }

    pub fn pop(&mut self) -> Result<u16> {
        let result = self.memory.read_word(self.cpu.sp);
        self.cpu.sp = self.cpu.sp.wrapping_add(2);
        result
    }

    pub fn interrupt(&mut self, port: u8) -> Result<()> {
        self.push(self.cpu.pc)?;
        self.push(self.cpu.flags)?;
        self.push(self.cpu.a)?;
        self.push(self.cpu.b)?;
        self.push(self.cpu.c)?;
        self.push(self.cpu.d)?;
        self.cpu.d = 0xFE00 + port as u16 * 2;
        self.cpu.flags = Flag::EnableInterrupt.to_bitmask();
        self.cpu.pc = self.memory.read_word(0xFE00 + port as u16 * 2)?;
        self.ports.acknowledge(port)?;
        Ok(())
    }
}
