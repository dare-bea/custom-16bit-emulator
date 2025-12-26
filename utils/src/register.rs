#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
#[repr(u8)]
pub enum Register {
    A = 0, B, C, D, Sp = 5, Pc, Flags,
}

impl From<&Register> for u8 {
    fn from(value: &Register) -> Self {
        use Register::*;
        match value {
            A => 0,
            B => 1,
            C => 2,
            D => 3,
            Sp => 5,
            Pc => 6,
            Flags => 7,
        }
    }
}

impl From<Register> for u8 {
    fn from(value: Register) -> Self {
        Self::from(&value)
    }
}

impl TryFrom<&u8> for Register {
    type Error = ();
    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        use Register::*;
        Ok(match value {
            0 => A,
            1 => B,
            2 => C,
            3 => D,
            5 => Sp,
            6 => Pc,
            7 => Flags,
            _ => return Err(())
        })
    }
}

impl TryFrom<u8> for Register {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

impl Register {
    pub fn pair_from(value: &u8) -> Result<(Self, Self), ()> {
        Ok(((value >> 4).try_into()?, (value & 0xF).try_into()?))
    }
}