mod condition {
    include!("./src/condition.rs");
}

use std::env;
use std::fs;
use std::path::Path;
use std::str::FromStr;
use condition::ConditionCode;

fn parse_operand(string: &str) -> Option<String> {
    if string.is_empty() {
        return None
    }
    Some(match string {
        "addr" => "OperandType::Address".into(),
        "dst" | "src" | "reg" => "OperandType::Register".into(),
        "#imm8" | "imm8" => "OperandType::Byte".into(),
        "#imm" | "#imm16" | "imm" | "imm16" => "OperandType::Word".into(),
        "rel" => "OperandType::Offset".into(),
        c => format!("OperandType::Const(\"{c}\")")
    })
}

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("instructions.in");
    let instructions = fs::read_to_string("instructions.tsv").unwrap();
    let mut result = String::from("&[\n");

    for line in instructions.lines().skip(1) {
        // Op	 Mnemonic	 Operand 1	 Operand 2	 Bytes	 Description
        // May contain condition code in mnemonic
        let mut parts = line.split('\t');
        let opcode = u8::from_str_radix(parts.next().expect("Expected opcode"), 16).expect("Invalid opcode");
        let mnem = parts.next().expect("Expected mnemonic");
        if mnem.is_empty() || mnem.starts_with('#') {
            continue;
        }
        let op1 = parse_operand(parts.next().unwrap_or(""));
        let op2 = parse_operand(parts.next().unwrap_or(""));
        let _bytes = parts.next();
        let _desc = parts.next();
        for cc in if mnem.contains("{cc}") {
            vec![
                "B",  "BE",  "EQ",  "AE",  "A",
                "NB", "NBE", "NEQ", "NAE", "NA",
                "L",  "LE",  "E",   "G",   "GE",
                "NL", "NLE", "NE",  "NG",  "NGE",
                "LT", "GE",  "NLT", "NGE",
                "Z",  "S",   "C",   "O",
                "NZ", "NS",  "NC",  "NO",
            ].into_iter().map(Some).collect()
        } else {
            vec![None]
        } {
            result.push_str("  ");
            if let Some(cc) = cc {
                let cc_mnem = mnem.replace("{cc}", cc);
                result.push_str(&format!("({opcode}, \"{cc_mnem}\", &["));
                let cc_val = u8::from(ConditionCode::from_str(cc).unwrap());
                result.push_str(&format!("OperandType::Hidden({cc_val}), "));
            } else {
                result.push_str(&format!("({opcode}, \"{mnem}\", &["));
            }
            if let (Some("OperandType::Register"), Some("OperandType::Register")) = (op1.as_deref(), op2.as_deref()) {
                result.push_str("OperandType::RegisterPair, ");
            } else {
                if let Some(op1) = &op1 {
                    result.push_str(op1);
                    result.push_str(", ");
                }
                if let Some(op2) = &op2 {
                    result.push_str(op2);
                    result.push_str(", ");
                }
            }
            result.push_str("]),\n");
        }
        result.push_str("\n");
    }
    result.push_str("]");
    fs::write(dest_path, result).unwrap();
    println!("cargo:rerun-if-changed=instructions.tsv");
    println!("cargo:rerun-if-changed=build.rs");
}