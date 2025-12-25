pub mod step;

pub struct Cpu {
    pub a: u16,
    pub b: u16,
    pub c: u16,
    pub d: u16,
    pub sp: u16,
    pub pc: u16,
    pub flags: u16,
}

pub struct Memory;

pub struct Emulator {
    pub cpu: Cpu,
    pub memory: Memory,
}