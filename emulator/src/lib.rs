use cpu::Cpu;
use memory::Mmu;
use std::io::Read;
use std::io::Seek;
use std::{fmt::Debug, io};
use utils::flag::Flag;

pub mod cpu;
pub mod memory;
pub mod step;

use crate::memory::{Ram, SimpleRom};

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
        let mut buf = [0; 2];
        self.memory.seek(io::SeekFrom::Start(0xFFFE))?;
        self.memory.read(&mut buf)?;
        self.cpu.pc = u16::from_le_bytes(buf);
        if self.cpu.pc == 0 {
            self.cpu.flags |= Flag::Halt.to_bitmask(); // TODO: Add Display
        }
        Ok(())
    }
}
