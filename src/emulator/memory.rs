use std::io::{stdin, Read};
use crate::memory::Memory;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Cartridge {
    pub data: [u8; 0x8000],
    pub locked: bool,
}

impl Cartridge {
    pub fn unlock(&mut self) {
        self.locked = false;
    }

    pub fn lock(&mut self) {
        self.locked = true;
    }
}

impl Default for Cartridge {
    fn default() -> Self {
        Self {
            data: [0; 0x8000],
            locked: false, // Default to unlocked for initialization
        }
    }
}

impl Memory for Cartridge {
    fn read(&self, address: usize) -> u8 {
        self.data[address]
    }

    fn write(&mut self, address: usize, value: u8) {
        if !self.locked {
            self.data[address] = value;
        } else {
            eprintln!(
                "Attempted to write to locked ROM at address {:04x}",
                address
            );
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RAM<const N: usize> {
    pub data: [u8; N],
}

impl<const N: usize> Memory for RAM<N> {
    fn read(&self, address: usize) -> u8 {
        self.data[address]
    }

    fn write(&mut self, address: usize, value: u8) {
        self.data[address] = value;
    }
}

pub const RAM_SIZE: usize = 0x3000; // 12KB RAM

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MMU {
    pub ram: RAM<RAM_SIZE>,
    pub rom: Cartridge,
}

impl Memory for MMU {
    fn read(&self, address: usize) -> u8 {
        match address {
            0x4000..=0x6FFF => self.ram.read(address),
            0x7F00 => {
                // Memory-mapped I/O for input
                stdin()
                    .lock()
                    .bytes()
                    .next()
                    .and_then(|result| result.ok())
                    .unwrap_or(u8::MAX)
            }
            0x8000..=0xFFFF => self.rom.read(address - 0x8000),
            _ => panic!("Invalid read address {address:#X}"),
        }
    }

    fn write(&mut self, address: usize, value: u8) {
        match address {
            0x0000..RAM_SIZE => self.ram.write(address, value),
            0x7F00 => {
                // Memory-mapped I/O for printing characters
                print!("{}", value as char);
            }
            0x8000..0x10000 => self.rom.write(address - 0x8000, value),
            _ => panic!("Invalid write address {address:#X}"),
        }
    }
}