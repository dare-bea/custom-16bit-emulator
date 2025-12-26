use std::{fmt::Display, str::FromStr};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum Flag {
    Zero,
    Sign,
    Carry,
    Overflow,
    EnableInterrupt,
    Halt,
}

impl Flag {
    pub fn to_bitmask(&self) -> u16 {
        1u16 << u8::from(self)
    }
}

impl FromStr for Flag {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ZF" => Ok(Self::Zero),
            "SF" => Ok(Self::Sign),
            "CF" => Ok(Self::Carry),
            "OF" => Ok(Self::Overflow),
            "EIF" => Ok(Self::EnableInterrupt),
            "HLT" => Ok(Self::Halt),
            _ => Err(()),
        }
    }
}

impl Display for Flag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Zero => "ZF",
                Self::Sign => "SF",
                Self::Carry => "CF",
                Self::Overflow => "OF",
                Self::EnableInterrupt => "EIF",
                Self::Halt => "HLT",
            }
        )
    }
}

impl From<&Flag> for u8 {
    fn from(value: &Flag) -> Self {
        match value {
            Flag::Zero => 0,
            Flag::Sign => 1,
            Flag::Carry => 2,
            Flag::Overflow => 3,
            Flag::EnableInterrupt => 6,
            Flag::Halt => 7,
        }
    }
}

impl From<Flag> for u8 {
    fn from(value: Flag) -> Self {
        <u8 as From<&Flag>>::from(&value)
    }
}

impl TryFrom<&u8> for Flag {
    type Error = ();
    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Zero),
            1 => Ok(Self::Sign),
            2 => Ok(Self::Carry),
            3 => Ok(Self::Overflow),
            6 => Ok(Self::EnableInterrupt),
            7 => Ok(Flag::Halt),
            _ => Err(()),
        }
    }
}

impl TryFrom<u8> for Flag {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        <Flag as TryFrom<&u8>>::try_from(&value)
    }
}

pub fn set_flag<F>(status: &mut u16, flag: F, value: bool)
where
    u8: From<F>,
{
    if value {
        *status |= 1 << u8::from(flag);
    } else {
        *status &= !(1 << u8::from(flag));
    }
}

pub fn get_flag<F>(status: u16, flag: F) -> bool
where
    u8: From<F>,
{
    (status & (1 << u8::from(flag))) != 0
}
