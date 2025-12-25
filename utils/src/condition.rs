use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum ConditionCode {
    B,
    BE,
    AE,
    A,
    L,
    LE,
    GE,
    G,
    Z,
    S,
    C,
    O,
    NZ,
    NS,
    NC,
    NO,
}

impl FromStr for ConditionCode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "B" => ConditionCode::B,
            "BE" => ConditionCode::BE,
            "AE" => ConditionCode::AE,
            "A" => ConditionCode::A,
            "L" => ConditionCode::L,
            "LE" => ConditionCode::LE,
            "GE" => ConditionCode::GE,
            "G" => ConditionCode::G,
            "Z" => ConditionCode::Z,
            "S" => ConditionCode::S,
            "C" => ConditionCode::C,
            "O" => ConditionCode::O,
            "NB" => ConditionCode::AE,
            "NBE" => ConditionCode::A,
            "NAE" => ConditionCode::B,
            "NA" => ConditionCode::BE,
            "NL" => ConditionCode::GE,
            "NLE" => ConditionCode::G,
            "NGE" => ConditionCode::L,
            "NG" => ConditionCode::LE,
            "LT" => ConditionCode::L,
            "NLT" => ConditionCode::GE,
            "GT" => ConditionCode::G,
            "NGT" => ConditionCode::LE,
            "E" => ConditionCode::Z,
            "NE" => ConditionCode::NZ,
            "EQ" => ConditionCode::Z,
            "NEQ" => ConditionCode::NZ,
            "NZ" => ConditionCode::NZ,
            "NS" => ConditionCode::NS,
            "NC" => ConditionCode::NC,
            "NO" => ConditionCode::NO,
            _ => return Err(()),
        })
    }
}

impl ToString for ConditionCode {
    fn to_string(&self) -> String {
        String::from(match self {
            ConditionCode::B => "B",
            ConditionCode::BE => "BE",
            ConditionCode::AE => "AE",
            ConditionCode::A => "A",
            ConditionCode::L => "L",
            ConditionCode::LE => "LE",
            ConditionCode::GE => "GE",
            ConditionCode::G => "G",
            ConditionCode::Z => "Z",
            ConditionCode::S => "S",
            ConditionCode::C => "C",
            ConditionCode::O => "O",
            ConditionCode::NZ => "NZ",
            ConditionCode::NS => "NS",
            ConditionCode::NC => "NC",
            ConditionCode::NO => "NO",
        })
    }
}

impl From<ConditionCode> for u8 {
    fn from(value: ConditionCode) -> Self {
        match value {
            ConditionCode::B => 0x4,
            ConditionCode::BE => 0x5,
            ConditionCode::AE => 0xC,
            ConditionCode::A => 0xD,
            ConditionCode::L => 0x6,
            ConditionCode::LE => 0x7,
            ConditionCode::GE => 0xE,
            ConditionCode::G => 0xF,
            ConditionCode::Z => 0x0,
            ConditionCode::S => 0x1,
            ConditionCode::C => 0x2,
            ConditionCode::O => 0x3,
            ConditionCode::NZ => 0x8,
            ConditionCode::NS => 0x9,
            ConditionCode::NC => 0xA,
            ConditionCode::NO => 0xB,
        }
    }
}

impl TryFrom<u8> for ConditionCode {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x4 => Ok(ConditionCode::B),
            0x5 => Ok(ConditionCode::BE),
            0xC => Ok(ConditionCode::AE),
            0xD => Ok(ConditionCode::A),
            0x6 => Ok(ConditionCode::L),
            0x7 => Ok(ConditionCode::LE),
            0xE => Ok(ConditionCode::GE),
            0xF => Ok(ConditionCode::G),
            0x0 => Ok(ConditionCode::Z),
            0x1 => Ok(ConditionCode::S),
            0x2 => Ok(ConditionCode::C),
            0x3 => Ok(ConditionCode::O),
            0x8 => Ok(ConditionCode::NZ),
            0x9 => Ok(ConditionCode::NS),
            0xA => Ok(ConditionCode::NC),
            0xB => Ok(ConditionCode::NO),
            _ => Err(())
        }
    }
}