use std::io::{BufRead};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum Register {
    A = 0,
    B,
    C,
    D,
    SP = 5,
    PC,
    FLAG,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operand {
    Immediate8(u8),
    Immediate16(u16),
    Registers(Register, Register),
    RelativeAddress(i8),
    Address(u16),
    Label(&str),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Instruction {
    opcode: u8,
    operands: Vec<Operand>,
}

struct Label {
    name: String,
    address: u16,
}

enum Directive {
    Data(Vec<u8>),
}

enum AssemblyLine {
    Instruction(Instruction),
    Label(Label),
    Directive(Directive),
    Empty,
}

enum ParseError {
    IOError(line_no, std::io::Error),
    InvalidSyntax(line_no, String),
    UnknownInstruction(line_no, String),
    UnknownRegister(String),
    InvalidOperand(String),
}

pub fn compile(source: impl BufRead) -> Result<Vec<u8>, ParseError> {
    // Refer to [instructions.tsv](../instructions.tsv) for instruction encoding details.

    let mut asm_lines: Vec<AssemblyLine> = Vec::new();

    for line in source.lines() {
        let line = line.map_err(ParseError::IOError)?;
        let line = line.trim();

        // Syntax:
        // - label:
        // - instruction operand1, operand2, ...
        // - .directive data1, data2, ...
        // - empty line
        // - comments start with ';' and extend to the end of the line

        // Skip empty lines and comments
        if line.is_empty() || line.starts_with(';') {
            asm_lines.push(AssemblyLine::Empty);
            continue;
        }

        // Handle directives; they might have syntaxic elements like colons
        if line.starts_with(".ascii") {
            // Allow backslash-escaped strings
            let line = line.strip_prefix(".ascii").unwrap().trim();
            let data: Vec<u8> = Vec::new(); // Placeholder for actual parsing logic
            let mut char_idx = 0;
            while char_idx < line.len() {
                let c = line.as_bytes()[char_idx] as char;
                if c == '"' {
                    char_idx += 1;
                    while char_idx < line.len() {
                        let c = line.as_bytes()[char_idx] as char;
                        if c == '"' {
                            char_idx += 1;
                            break;
                        } else if c == '\\' {
                            char_idx += 1;
                            if char_idx >= line.len() {
                                return Err(ParseError::InvalidSyntax("Unfinished escape sequence".to_string()));
                            }
                            let esc = line.as_bytes()[char_idx] as char;
                            match esc {
                                'n' => data.push(b'\n'),
                                't' => data.push(b'\t'),
                                'r' => data.push(b'\r'),
                                '\\' => data.push(b'\\'),
                                '"' => data.push(b'"'),
                                other => return Err(ParseError::InvalidSyntax(format!("Unknown escape sequence: \\{}", other))),
                            }
                            char_idx += 1;
                        } else {
                            data.push(c as u8);
                            char_idx += 1;
                        }
                    }
                    return Err(ParseError::InvalidSyntax("Unterminated string literal".to_string()));
                }
            }
        }
    }

    todo!()
}