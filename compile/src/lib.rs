use std::{collections::HashMap, io::{self, BufRead, Cursor, Write}, str::FromStr};

use num::cast::AsPrimitive;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum InstructionError {
    InvalidNumber(String),
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

fn parse_symbol(string: &str) -> Result<String, ()> {
    if string.chars().all(|c|
        c.is_alphanumeric() || matches!(c, '_' | '-' | '.' | '+')
    ) && parse_register(string).is_err() {
        Ok(string.to_string())
    } else {
        Err(())
    }
}

enum Value<T> {
    Literal(T),
    Symbol(String)
}

fn parse_or_symbol<U: num::PrimInt + std::str::FromStr + 'static, I: num::PrimInt + num::traits::AsPrimitive<U> + std::str::FromStr>(string: &str) -> Result<Value<U>, InstructionError> {
    // Try conversion to u16, then try i16 (converted to u16 afterwards).
    match parse_integer::<U, I>(string) {
        Ok(value) => Ok(Value::Literal(value)),
        Err(_) => parse_symbol(string).map(Value::Symbol).map_err(|_| InstructionError::InvalidNumber(string.to_string()))
    }
}

fn parse_immediate(string: &str) -> Result<Value<u16>, InstructionError> {
    if let Some(string) = string.strip_prefix('#').or(string.strip_prefix("W#")) {
        parse_or_symbol::<u16, i16>(string)
    } else {
        Err(InstructionError::InvalidImmediate(string.to_string()))
    }
}

fn parse_immediate8(string: &str) -> Result<Value<u8>, InstructionError> {
    if let Some(string) = string.strip_prefix('#').or(string.strip_prefix("B#")) {
        match parse_integer::<u8, i8>(string) {
            Ok(value) => Ok(Value::Literal(value)),
            Err(_) => Err(InstructionError::InvalidNumber(string.to_string())), // does not accept symbols
        }
    } else {
        Err(InstructionError::InvalidImmediate(string.to_string()))
    }
}

#[allow(dead_code)]
fn parse_immediate8_symbol(string: &str) -> Result<Value<u8>, InstructionError> {
    if let Some(string) = string.strip_prefix('#').or(string.strip_prefix("B#")) {
        parse_or_symbol::<u8, i8>(string)
    } else {
        Err(InstructionError::InvalidImmediate(string.to_string()))
    }
}

fn parse_address(string: &str) -> Result<Value<u16>, InstructionError> {
    parse_or_symbol::<u16, i16>(string)
}

fn parse_offset(string: &str) -> Result<Value<i8>, InstructionError> {
    parse_or_symbol::<i8, u8>(string)
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
    let line = line.split_once(';').unwrap_or((line, "")).0; // get rid of comments
    let ops: Vec<&str> = line.split(',').map(|x| x.trim()).filter(|x| !x.is_empty()).collect();

    // find bytes for instruction
    let mut last_err: (usize, InstructionError) = (0, InstructionError::InvalidMnemonic(mnem.clone()));
    'outer: for instruction in INSTRUCTIONS {
        if instruction.1 != mnem {
            continue;
        }
        #[cfg(debug_assertions)]
        eprintln!("{instruction:?} | {mnem:?} | {ops:?}");
        let mut bytes = vec![instruction.0];
        let mut ops = ops.iter();
        let mut symbols: Vec<(u64, String, usize)> = Vec::new();
        'inner: for optype in instruction.2 {
            #[cfg(debug_assertions)]
            eprintln!("{optype:?}");
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
                    Ok(Value::Literal(addr)) => bytes.extend_from_slice(&addr.to_le_bytes()),
                    Ok(Value::Symbol(sym)) => {
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
                    Ok(Value::Literal(offset)) => bytes.push(offset as u8),
                    Ok(Value::Symbol(sym)) => {
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
                    Ok(Value::Literal(byte)) => bytes.push(byte),
                    Ok(Value::Symbol(sym)) => {
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
                    Ok(Value::Literal(word)) => bytes.extend_from_slice(&word.to_le_bytes()),
                    Ok(Value::Symbol(sym)) => {
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
    SymbolsNotSupported(String),
    UndefinedSymbol(String),
    SymbolOutOfRange(u64, )
}

fn parse_directive(directive: &str, line: &str, binary: &mut Cursor<Vec<u8>>, symbols: &mut HashMap<String, u64>, errata: &mut Vec<(u64, String, usize)>) -> Result<(), DirectiveError> {
    match directive {
        "def" => {
            let mut operands = line.split_whitespace();
            let sym = operands.next().ok_or(DirectiveError::MissingOperand("symbol".to_string()))?.trim();
            let string = operands.next().ok_or(DirectiveError::MissingOperand("value".to_string()))?.trim();
            if let Some(s) = operands.next() {return Err(DirectiveError::ExtraOperand(s.to_string()))};

            let value = match parse_or_symbol::<u16, i16>(string).map_err(|_| DirectiveError::InvalidNumber(string.to_string()))? {
                Value::Literal(x) => x as u64,
                Value::Symbol(s) => match symbols.get(&s) {
                    Some(x) => *x,
                    None => return Err(DirectiveError::UndefinedSymbol(s)),
                }
            };

            symbols.insert(sym.to_string(), value);
        }
        "db" => for string in line.split_whitespace() {
            let string = string.trim();
            match parse_or_symbol::<u8, i8>(string).map_err(|_| DirectiveError::InvalidNumber(string.to_string()))? {
                Value::Literal(value) => {
                    binary.write_all(&[value]).map_err(DirectiveError::IOError)?;
                },
                Value::Symbol(s) => {
                    errata.push((binary.position(), s, 1));
                    binary.write_all(&[0]).map_err(DirectiveError::IOError)?;
                }
            };
        }
        "dw" => for string in line.split_whitespace() {
            let string = string.trim();
            match parse_or_symbol::<u16, i16>(string).map_err(|_| DirectiveError::InvalidNumber(string.to_string()))? {
                Value::Literal(value) => {
                    binary.write_all(&value.to_le_bytes()).map_err(DirectiveError::IOError)?;
                },
                Value::Symbol(s) => {
                    errata.push((binary.position(), s, 2));
                    binary.write_all(&[0, 0]).map_err(DirectiveError::IOError)?;
                }
            };
        }
        "org" => {
            let string = line.trim();
            let value = match parse_or_symbol::<u16, i16>(string).map_err(|_| DirectiveError::InvalidNumber(string.to_string()))? {
                Value::Literal(x) => x as u64,
                Value::Symbol(s) => match symbols.get(&s) {
                    Some(x) => *x,
                    None => return Err(DirectiveError::UndefinedSymbol(s)),
                }
            };
            binary.set_position(value - START_ADDRESS);
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
    SymbolOutOfRange(String, usize)
}

const START_ADDRESS: u64 = 0x8000;

pub fn compile(source: impl BufRead) -> Result<Vec<u8>, (Option<usize>, CompileError)> {
    let mut binary: Cursor<Vec<u8>> = Cursor::default();
    let mut symbols: HashMap<String, u64> = HashMap::new();
    let mut errata: Vec<(u64, String, usize)> = Vec::new();
    for (line_no, line) in source.lines().enumerate() {
        #[cfg(debug_assertions)]
        eprintln!("## line #{line_no} ##");
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
        let bytes = match symbols.get(&sym) {
            Some(value) => value.to_le_bytes(),
            None => return Err((None, CompileError::UndefinedSymbol(sym))),
        };
        let bytes = &bytes[..len];
        if bytes[len..].iter().any(|x| !matches!(x, 0x00 | 0xFF)) {
            return Err((None, CompileError::SymbolOutOfRange(sym, 1usize << len)));
        }
        binary.write_all(bytes).map_err(|e| (None, CompileError::IOError(e)))?;
    }
    Ok(binary.into_inner())
}