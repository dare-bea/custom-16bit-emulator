use crate::memory::Memory;
pub mod execution;

pub struct Cartridge {
    data: [u8; 0x8000],
}

impl Memory for Cartridge {
    fn read(&self, address: usize) -> u8 {
        self.data[address]
    }

    fn write(&mut self, _address: usize, _value: u8) {
        // ROM is read-only, so we do nothing here
    }
}

pub struct RAM<const N: usize> {
    data: [u8; N],
}

impl<const N: usize> Memory for RAM<N> {
    fn read(&self, address: usize) -> u8 {
        self.data[address]
    }

    fn write(&mut self, address: usize, value: u8) {
        self.data[address] = value;
    }
}

pub struct MMU {
    pub rom: Cartridge,
    pub ram: RAM<0x2000>,
}

impl Memory for MMU {
    fn read(&self, address: usize) -> u8 {
        match address {
            0x0000..0x2000 => self.ram.read(address),
            0x8000..0x10000 => self.rom.read(address - 0x8000),
            _ => panic!("Invalid address"),
        }
    }

    fn write(&mut self, address: usize, value: u8) {
        match address {
            0x0000..0x2000 => self.ram.write(address, value),
            0x8000..0x10000 => self.rom.write(address - 0x8000, value),
            _ => panic!("Invalid address"),
        }
    }
}

pub struct CPU {
    pub a: u16,
    pub b: u16,
    pub c: u16,
    pub d: u16,
    pub pc: u16,
    pub sp: u16,
    pub flags: u8,
    pub ir_flags: u16,
}

pub struct Emulator {
    pub memory: MMU,
    pub cpu: CPU,
}