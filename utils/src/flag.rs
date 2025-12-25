use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum Flag {
    Zero,
    Sign,
    Carry,
    Overflow,
    EnableInterrupt,
    Halt,
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

impl ToString for Flag {
    fn to_string(&self) -> String {
        match self {
            Self::Zero => "ZF",
            Self::Sign => "SF",
            Self::Carry => "CF",
            Self::Overflow => "OF",
            Self::EnableInterrupt => "EIF",
            Self::Halt => "HLT",
        }
        .to_string()
    }
}

impl From<Flag> for u8 {
    fn from(value: Flag) -> Self {
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

impl TryFrom<u8> for Flag {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
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

pub fn set_flag<F>(status: &mut u8, flag: F, value: bool)
where
    u8: From<F>,
{
    if value {
        *status |= 1 << u8::from(flag);
    } else {
        *status &= !(1 << u8::from(flag));
    }
}

pub fn get_flag<F>(status: u8, flag: F) -> bool
where
    u8: From<F>,
{
    (status & (1 << u8::from(flag))) != 0
}
