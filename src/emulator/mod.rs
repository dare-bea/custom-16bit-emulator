use crate::memory::Memory;
pub mod execution;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Cartridge {
    data: [u8; 0x8000],
    locked: bool,
}

impl Cartridge {
    pub fn unlock(&mut self) {
        self.locked = false;
    }

    pub fn lock(&mut self) {
        self.locked = true;
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

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MMU {
    pub ram: RAM<0x2000>,
    pub rom: Cartridge,
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
            0x6000 => {
                // Memory-mapped I/O for printing characters
                print!("{}", value as char);
            }
            0x8000..0x10000 => self.rom.write(address - 0x8000, value),
            _ => panic!("Invalid address"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Emulator {
    pub memory: MMU,
    pub cpu: CPU,
}

impl Default for Emulator {
    fn default() -> Self {
        Self::new()
    }
}

impl Emulator {
    pub fn new() -> Self {
        let mut emu = Emulator {
            memory: MMU {
                rom: Cartridge {
                    data: [0; 0x8000],
                    locked: true,
                },
                ram: RAM { data: [0; 0x2000] },
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

    pub fn from_rom(rom_data: &[u8]) -> Self {
        let mut emu = Emulator::new();
        emu.memory.rom.load(0, rom_data);
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
