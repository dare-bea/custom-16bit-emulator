use crate::memory::Memory;

pub mod cpu;
pub mod execution;
pub mod memory;

use cpu::CPU;
use memory::{Cartridge, MMU, RAM};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Emulator {
    pub memory: MMU,
    pub cpu: CPU,
}

impl Default for Emulator {
    fn default() -> Self {
        Self::new(Cartridge::default())
    }
}

impl Emulator {
    pub fn new(rom: Cartridge) -> Self {
        let mut emu = Emulator {
            memory: MMU {
                rom,
                ram: RAM { data: [0; 0x3000] },
            },
            cpu: CPU {
                a: 0,
                b: 0,
                c: 0,
                d: 0,
                pc: 0,
                sp: 0x1FFF,
                flags: 0,
                ir_flags: 0,
            },
        };
        emu.reset();
        emu
    }

    pub fn reset(&mut self) {
        self.cpu.a = 0;
        self.cpu.b = 0;
        self.cpu.c = 0;
        self.cpu.d = 0;
        self.cpu.sp = 0x1FFF;
        self.cpu.flags = 0;
        self.cpu.ir_flags = 0;

        // Reset Vector
        self.cpu.pc = self.memory.read_word(0xFFFE);
    }

    pub fn is_running(&self) -> bool {
        (self.cpu.flags & (1 << crate::flag::HALT)) == 0
    }
}
