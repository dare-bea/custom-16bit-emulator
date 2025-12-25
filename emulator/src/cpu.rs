#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Cpu {
    pub a: u16,
    pub b: u16,
    pub c: u16,
    pub d: u16,
    pub sp: u16,
    pub pc: u16,
    pub flags: u16,
}

impl Default for Cpu {
    fn default() -> Self {
        return Self {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            sp: 0x7F00,
            pc: 0x0000,
            flags: 0,
        };
    }
}

impl Cpu {
    pub fn new() -> Self {
        Self::default()
    }
}
