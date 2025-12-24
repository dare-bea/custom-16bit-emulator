use crate::register::Register;

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

impl CPU {
    pub fn register(&self, reg: Register) -> u16 {
        match reg {
            Register::A => self.a,
            Register::B => self.b,
            Register::C => self.c,
            Register::D => self.d,
        }
    }

    pub fn mut_register(&mut self, reg: Register) -> &mut u16 {
        match reg {
            Register::A => &mut self.a,
            Register::B => &mut self.b,
            Register::C => &mut self.c,
            Register::D => &mut self.d,
        }
    }
}
