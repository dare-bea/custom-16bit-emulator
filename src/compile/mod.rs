use std::{collections::HashMap, io::{self, BufRead, Cursor, Write}, str::FromStr};

use num::cast::AsPrimitive;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum InstructionError {
    ExpectedMemonic,
    InvalidNumber(std::num::ParseIntError),
    InvalidImmediate(String),
    InvalidRegister(String),
    InvalidOffset(String),
    InvalidMnemonic(String),
    InvalidConstant{expected: String, found: String},
    MissingOperand(String),
    ExtraOperand(String),
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

fn parse_number(string: &str) -> Result<Result<u16, String>, InstructionError> {
    // Try conversion to u16, then try i16 (converted to u16 afterwards).
    let result = if let Some(string) = string.strip_prefix('$') {
        u16::from_str_radix(string, 16)
            .or_else(|_| i16::from_str_radix(string, 16).map(|x| x as u16))
    } else if let Some(string) = string.strip_prefix('%') {
        u16::from_str_radix(string, 2).or_else(|_| i16::from_str_radix(string, 2).map(|x| x as u16))
    } else {
        string
            .parse::<u16>()
            .or_else(|_| string.parse::<i16>().map(|x| x as u16))
    };
    match result {
        Ok(addr) => Ok(Ok(addr)),
        Err(_) if string.chars().all(|c|
            c.is_alphanumeric() || matches!(c, '_' | '-' | '.' | '+')
        ) => Ok(Err(string.to_string())),
        Err(err) => Err(InstructionError::InvalidNumber(err)),
    }
}

fn parse_immediate(string: &str) -> Result<Result<u16, String>, InstructionError> {
    if let Some(string) = string.strip_prefix('#') {
        parse_number(string)
    } else {
        Err(InstructionError::InvalidImmediate(string.to_string()))
    }
}

fn parse_immediate8(string: &str) -> Result<Result<u8, String>, InstructionError> {
    match parse_immediate(string).map(|x|
        x.map(|x| u8::try_from(x)))
    {
        Ok(Ok(Ok(value))) => Ok(Ok(value)),
        Ok(Ok(Err(_))) => Err(InstructionError::InvalidImmediate(string.to_string())),
        Ok(Err(s)) => Ok(Err(s)),
        Err(err) => Err(err),
    }
}

fn parse_address(string: &str) -> Result<Result<u16, String>, InstructionError> {
    parse_number(string)
}

fn parse_offset(string: &str) -> Result<Result<i8, String>, InstructionError> {
    match parse_number(string).map(|x|
        x.map(|x| i8::try_from(x as i16)))
    {
        Ok(Ok(Ok(value))) => Ok(Ok(value)),
        Ok(Ok(Err(_))) => Err(InstructionError::InvalidImmediate(string.to_string())),
        Ok(Err(s)) => Ok(Err(s)),
        Err(err) => Err(err),
    }
}

fn parse_register(string: &str) -> Result<u8, InstructionError> {
    match string.to_uppercase().as_str() {
        "A" => Ok(Register::A as u8),
        "B" => Ok(Register::B as u8),
        "C" => Ok(Register::C as u8),
        "D" => Ok(Register::D as u8),
        "SP" => Ok(Register::SP as u8),
        "PC" => Ok(Register::PC as u8),
        "FLAGS" => Ok(Register::FLAGS as u8),
        _ => Err(InstructionError::InvalidRegister(string.to_string())),
    }
}

fn parse_register_pair(string1: &str, string2: &str) -> Result<u8, InstructionError> {
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
    Hidden(u8),
}

impl ToString for OperandType {
    fn to_string(&self) -> String {
        match self {
            Self::Address => String::from("addr"),
            Self::Offset => String::from("rel"),
            Self::Byte => String::from("#imm8"),
            Self::Word => String::from("#imm16"),
            Self::Register => String::from("reg"),
            Self::RegisterPair => String::from("dst, src"),
            Self::Const(s) => String::from(*s),
            Self::Hidden(x) => format!("({x})"),
        }
    }
}

const INSTRUCTIONS: &[(u8, &str, &[OperandType])] =
    include!(concat!(env!("OUT_DIR"), "/instructions.in"));

struct InstructionEmission {
    bytes: Vec<u8>,
    symbols: Vec<(u64, String, usize)>
}

fn parse_instruction(line: &str) -> Result<InstructionEmission, InstructionError> {
    // S* ident S+ ([operand S* "," S*] (operand)) [";" comment]

    let line = line.trim_start();
    if line.starts_with(';') || line.is_empty() {
        return Ok(InstructionEmission {bytes: vec![], symbols: vec![]})
    }
    let (mnem, line) = line.split_once(char::is_whitespace).unwrap_or((line, ""));
    let mnem = mnem.to_uppercase();
    let mut ops: Vec<&str> = Vec::new();
    let mut line = line;

    loop {
        line = line.trim_start();
        if line.starts_with(';') || line.is_empty() {
            break;
        }
        let operand;
        (operand, line) = line.split_once([',', ';']).unwrap_or((line, ""));
        ops.push(operand.trim());
    }

    // find bytes for instruction
    let mut last_err: (usize, InstructionError) = (0, InstructionError::InvalidMnemonic(mnem.clone()));
    'outer: for instruction in INSTRUCTIONS {
        if instruction.1 != mnem {
            continue;
        }
        let mut bytes = vec![instruction.0];
        let mut ops = ops.iter();
        let mut symbols: Vec<(u64, String, usize)> = Vec::new();
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
                OperandType::Address => match parse_address(op) {
                    Ok(Ok(addr)) => bytes.extend_from_slice(&addr.to_le_bytes()),
                    Ok(Err(sym)) => {
                        symbols.push((bytes.len() as u64, sym, 2));
                        bytes.extend_from_slice(&[0, 0])
                    },
                    Err(e) => {
                        if last_err.0 < bytes.len() {
                            last_err = (bytes.len(), e)
                        }
                        continue 'outer;
                    }
                },
                OperandType::Offset => match parse_offset(op) {
                    Ok(Ok(offset)) => bytes.push(offset as u8),
                    Ok(Err(sym)) => {
                        symbols.push((bytes.len() as u64, sym, 1));
                        bytes.extend_from_slice(&[0])
                    },
                    Err(e) => {
                        if last_err.0 < bytes.len() {
                            last_err = (bytes.len(), e)
                        }
                        continue 'outer;
                    }
                },
                OperandType::Byte => match parse_immediate8(op) {
                    Ok(Ok(byte)) => bytes.push(byte),
                    Ok(Err(sym)) => {
                        symbols.push((bytes.len() as u64, sym, 1));
                        bytes.extend_from_slice(&[0])
                    },
                    Err(e) => {
                        if last_err.0 < bytes.len() {
                            last_err = (bytes.len(), e)
                        }
                        continue 'outer;
                    }
                },
                OperandType::Word => match parse_immediate(op) {
                    Ok(Ok(word)) => bytes.extend_from_slice(&word.to_le_bytes()),
                    Ok(Err(sym)) => {
                        symbols.push((bytes.len() as u64, sym, 2));
                        bytes.extend_from_slice(&[0, 0])
                    },
                    Err(e) => {
                        if last_err.0 < bytes.len() {
                            last_err = (bytes.len(), e)
                        }
                        continue 'outer;
                    }
                },
                OperandType::Register => match parse_register(op) {
                    Ok(reg) => bytes.push(reg as u8),
                    Err(e) => {
                        if last_err.0 < bytes.len() {
                            last_err = (bytes.len(), e)
                        }
                        continue 'outer;
                    }
                },
                OperandType::RegisterPair => match parse_register_pair(
                    op,
                    match ops.next() {
                        Some(op) => op,
                        None => {
                            if last_err.0 < bytes.len() {
                                last_err =
                                    (bytes.len(), InstructionError::MissingOperand("src".to_string()))
                            }
                            continue 'outer;
                        }
                    },
                ) {
                    Ok(reg) => bytes.push(reg as u8),
                    Err(e) => {
                        if last_err.0 < bytes.len() {
                            last_err = (bytes.len(), e)
                        }
                        continue 'outer;
                    }
                },
                OperandType::Const(val) => {
                    if op != val {
                        if last_err.0 < bytes.len() {
                            last_err = (bytes.len(), InstructionError::InvalidConstant{expected: val.to_string(), found: op.to_string()})
                        }
                        continue 'outer;
                    }
                }
            }
        }
        match ops.next() {
            Some(o) => {
                if last_err.0 < bytes.len() {
                    last_err = (bytes.len(), InstructionError::ExtraOperand(o.to_string()))
                }
                continue 'outer;
            }
            None => {}
        }
        return Ok(InstructionEmission { bytes, symbols });
    }
    Err(last_err.1)
}

fn parse_label<'a>(line: &'a str) -> Option<(&'a str, &'a str)>{
    let line = line.trim_start();
    line.split_once(':')
}

#[derive(Debug)]
pub enum DirectiveError {
    IOError(io::Error),
    UnknownDirective(String),
    InvalidNumber(String),
    MissingOperand(String),
    ExtraOperand(String),
    ExpectedString(String),
}

fn parse_integer<U: num::PrimInt + FromStr + 'static, I: num::PrimInt + FromStr + 'static + AsPrimitive<U>>(string: &str) -> Result<U, <I as num::Num>::FromStrRadixErr> {
    if let Some(string) = string.strip_prefix('$') {
        U::from_str_radix(string, 16)
            .or_else(|_| I::from_str_radix(string, 16).map(|x| x.as_()))
    } else if let Some(string) = string.strip_prefix('%') {
        U::from_str_radix(string, 2).or_else(|_| I::from_str_radix(string, 2).map(|x| x.as_()))
    } else {
        U::from_str_radix(string, 10).or_else(|_| I::from_str_radix(string, 10).map(|x| x.as_()))
    }
}

fn parse_directive(directive: &str, line: &str, binary: &mut Cursor<Vec<u8>>, symbols: &mut HashMap<String, u64>, _errata: &mut Vec<(u64, String, usize)>) -> Result<(), DirectiveError> {
    match directive {
        "def" => {
            let mut operands = line.split_whitespace();
            let sym = operands.next().ok_or(DirectiveError::MissingOperand("symbol".to_string()))?.trim();
            let string = operands.next().ok_or(DirectiveError::MissingOperand("value".to_string()))?.trim();
            if let Some(s) = operands.next() {return Err(DirectiveError::ExtraOperand(s.to_string()))};

            let value = parse_integer::<u16, i16>(string).map_err(|_| DirectiveError::InvalidNumber(string.to_string()))?;

            symbols.insert(sym.to_string(), value as u64);
        }
        "db" => for string in line.split_whitespace() {
            let string = string.trim();
            let num = parse_integer::<u8, i8>(string).map_err(|_| DirectiveError::InvalidNumber(string.to_string()))?;
            binary.write_all(&[num]).map_err(DirectiveError::IOError)?;
        }
        "dw" => for string in line.split_whitespace() {
            let string = string.trim();
            let num = parse_integer::<u16, i16>(string).map_err(|_| DirectiveError::InvalidNumber(string.to_string()))?;
            binary.write_all(&num.to_le_bytes()).map_err(DirectiveError::IOError)?;
        }
        "org" => {
            let address = parse_integer::<u64, i64>(line.trim()).map_err(|_| DirectiveError::InvalidNumber(line.to_string()))? - START_ADDRESS;
            binary.set_position(address);
        }
        "ascii" => {
            let string = line.trim().strip_prefix('"').and_then(|l| l.strip_suffix('"')).ok_or(DirectiveError::ExpectedString(line.to_string()))?;
            binary.write_all(string.as_bytes()).map_err(DirectiveError::IOError)?;
        }
        _ => return Err(DirectiveError::UnknownDirective(directive.to_string()))
    };
    Ok(())
}

#[derive(Debug)]
pub enum CompileError {
    IOError(io::Error),
    InstructionError(InstructionError),
    DirectiveError(DirectiveError),
    UndefinedSymbol(String),
}

const START_ADDRESS: u64 = 0x8000;

pub fn compile(source: impl BufRead) -> Result<Vec<u8>, (Option<usize>, CompileError)> {
    let mut binary: Cursor<Vec<u8>> = Cursor::default();
    let mut symbols: HashMap<String, u64> = HashMap::new();
    let mut errata: Vec<(u64, String, usize)> = Vec::new();
    for (line_no, line) in source.lines().enumerate() {
        let mut line = line.map_err(|e| (Some(line_no), CompileError::IOError(e)))?;
        while let Some((label, rest)) = parse_label(&line) {
            symbols.insert(label.to_string(), START_ADDRESS + binary.position());
            line = rest.to_string();
        }
        line = line.trim_start().to_string();
        if let Some(l) = line.strip_prefix('.') {
            let (d, l) = l.split_once(' ').unwrap_or((l, ""));
            parse_directive(d, l, &mut binary, &mut symbols, &mut errata).map_err(|e| (Some(line_no), CompileError::DirectiveError(e)))?
        } else {
            let InstructionEmission { bytes: buf, symbols: missing } = parse_instruction(&line).map_err(|e| (Some(line_no), CompileError::InstructionError(e)))?;
            errata.extend(missing.into_iter().map(|(i, s, n)| (i + binary.position(), s, n)));
            binary.write_all(&buf).map_err(|e| (Some(line_no), CompileError::IOError(e)))?;
        }
    }
    for (pos, sym, len) in errata {
        binary.set_position(pos);
        let bytes: Vec<u8> = symbols.get(&sym).ok_or((None, CompileError::UndefinedSymbol(sym)))?.to_le_bytes().into_iter().take(len).collect();
        binary.write_all(&bytes).map_err(|e| (None, CompileError::IOError(e)))?;
    }
    Ok(binary.into_inner())
}