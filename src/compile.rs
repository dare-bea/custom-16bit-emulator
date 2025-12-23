#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    InvalidNumber(std::num::ParseIntError),
    InvalidImmediate(String),
    InvalidRegister(String),
    InvalidOffset(String),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
#[repr(u8)]
pub enum Register {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    SP = 5,
    PC = 6,
    FLAGS = 7,
}

impl std::fmt::Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Register::A => write!(f, "A"),
            Register::B => write!(f, "B"),
            Register::C => write!(f, "C"),
            Register::D => write!(f, "D"),
            Register::SP => write!(f, "SP"),
            Register::PC => write!(f, "PC"),
            Register::FLAGS => write!(f, "FLAGS"),
        }
    }
}

fn parse_number(string: &str) -> Result<u16, ParseError> {
    if let Some(string) = string.strip_prefix('$') {
        u16::from_str_radix(string, 16)
        .or_else(|_| i16::from_str_radix(string, 16).map(|x| x as u16))
    } else if let Some(string) = string.strip_prefix('%') {
        u16::from_str_radix(string, 2)
        .or_else(|_| i16::from_str_radix(string, 2).map(|x| x as u16))
    } else if let Some(string) = string.strip_prefix("0") {
        u16::from_str_radix(string, 8)
        .or_else(|_| i16::from_str_radix(string, 8).map(|x| x as u16))
    } else {
        string.parse::<u16>()
        .or_else(|_| string.parse::<i16>().map(|x| x as u16))
    }.map_err(|err| ParseError::InvalidNumber(err))
}

fn parse_immediate(string: &str) -> Result<u16, ParseError> {
    if let Some(string) = string.strip_prefix('#') {
        parse_number(string)
    } else {
        Err(ParseError::InvalidImmediate(string.to_string()))
    }
}

fn parse_immediate8(string: &str) -> Result<u8, ParseError> {
    match parse_immediate(string).map(|x| u8::try_from(x))
    {
        Ok(Ok(value)) => Ok(value),
        Ok(Err(_)) => Err(ParseError::InvalidImmediate(string.to_string())),
        Err(err) => Err(err),
    }
}

fn parse_offset(string: &str) -> Result<i8, ParseError> {
    match parse_number(string).map(|x| i8::try_from(x as i16)) {
        Ok(Ok(value)) => Ok(value),
        Ok(Err(_)) => Err(ParseError::InvalidOffset(string.to_string())),
        Err(err) => Err(err),
    }
}

fn parse_register(string: &str) -> Result<u8, ParseError> {
    match string.to_uppercase().as_str() {
        "A" => Ok(Register::A as u8),
        "B" => Ok(Register::B as u8),
        "C" => Ok(Register::C as u8),
        "D" => Ok(Register::D as u8),
        "SP" => Ok(Register::SP as u8),
        "PC" => Ok(Register::PC as u8),
        "FLAGS" => Ok(Register::FLAGS as u8),
        _ => Err(ParseError::InvalidRegister(string.to_string())),
    }
}

fn parse_register_pair(string1: &str, string2: &str) -> Result<u8, ParseError> {
    Ok(parse_register(string1)? << 4 | parse_register(string2)?)
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum OperandType {
    Address,
    Offset,
    Byte,
    Word,
    Register,
    RegisterPair,
    Const(&'static str),
    Hidden(u8)
}

const INSTRUCTIONS: &[(u8, &str, &[OperandType])] = include!(concat!(env!("OUT_DIR"), "/instructions.in"));

fn parse_instruction(line: &str) -> Option<Vec<u8>> {
    // S* ident S+ [operand] S* [, S* operand] S* [";" comment]

    let line = line.trim_start();
    // ident S+ (operand S* [, S* operand]) S* [";" comment]
    let (mnem, line) = line.split_once(char::is_whitespace).unwrap_or((line, ""));
    let mnem = &mnem.to_uppercase();
    let mut ops = Vec::new();
    let line = line.trim_start();
    if line.is_empty() || line.starts_with(';') {
        break;
    }
    // [operand] S* [, S* operand] S* [";" comment]
    match line.split_once(',') {
        Some((op, line)) => {
            ops.push(op.trim_end());
            let line = line.trim_start();
            if line.is_empty() || line.starts_with(';') {
                break;
            }
            // operand S* [";" comment]
            ops.push(line.split_once(';').map(|(op, _)| op.trim()).unwrap_or(line));
        }
        None => ops.push(line.split_once(';').map(|(op, _)| op.trim()).unwrap_or(line)),
    }

    // find bytes for instruction
    'outer: for instruction in INSTRUCTIONS {
        if instruction.1 != mnem {continue;}
        let mut bytes = vec![instruction.0];
        let mut ops = ops.iter();
        'inner: for optype in instruction.2 {
            if let OperandType::Hidden(val) = optype {
                bytes.push(*val);
                continue 'inner;
            }
            let op = match ops.next() {
                Some(op) => op,
                None => continue 'outer,
            };
            match optype {
                OperandType::Hidden(_) => unreachable!(),
                OperandType::Address => match parse_number(op) {
                    Ok(addr) => bytes.extend_from_slice(&addr.to_le_bytes()),
                    Err(_) => continue 'outer
                }
                OperandType::Offset => match parse_offset(op) {
                    Ok(offset) => bytes.push(offset as u8),
                    Err(_) => continue 'outer
                }
                OperandType::Byte => match parse_immediate8(op) {
                    Ok(byte) => bytes.push(byte),
                    Err(_) => continue 'outer
                }
                OperandType::Word => match parse_immediate(op) {
                    Ok(word) => bytes.extend_from_slice(&word.to_le_bytes()),
                    Err(_) => continue 'outer
                }
                OperandType::Register => match parse_register(op) {
                    Ok(reg) => bytes.push(reg as u8),
                    Err(_) => continue 'outer
                }
                OperandType::RegisterPair => match parse_register_pair(op, match ops.next() {
                    Some(op) => op,
                    None => continue 'outer
                }) {
                    Ok(reg) => bytes.push(reg as u8),
                    Err(_) => continue 'outer
                }
                OperandType::Const(val) => if op != val {continue 'outer;}
            }
        }
        return Some(bytes);
    }
    None
}

pub fn compile_line(line: &str) -> Option<Vec<u8>> {
    let line = line.trim();
    match line.split_once(' ') {
        Some((mnem, ops)) => parse_instruction(mnem, &ops.split(',').collect::<Vec<_>>()),
        None => parse_instruction(line, &[]),
    }
}