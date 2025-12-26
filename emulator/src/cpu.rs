use utils::register::Register;

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
        Self {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            sp: 0x7F00,
            pc: 0x0000,
            flags: 0,
        }
    }
}

impl Cpu {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&self, reg: Register) -> u16 {
        use Register::*;
        match reg {
            A => self.a,
            B => self.b,
            C => self.c,
            D => self.d,
            Sp => self.sp,
            Pc => self.pc,
            Flags => self.flags,
        }
    }

    pub fn register_mut(&mut self, reg: Register) -> &mut u16 {
        use Register::*;
        match reg {
            A => &mut self.a,
            B => &mut self.b,
            C => &mut self.c,
            D => &mut self.d,
            Sp => &mut self.sp,
            Pc => &mut self.pc,
            Flags => &mut self.flags,
        }
    }
}
