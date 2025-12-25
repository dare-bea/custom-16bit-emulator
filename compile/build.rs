use std::env;
use std::fs;
use std::path::Path;
use std::str::FromStr;
use utils::condition::ConditionCode;
use utils::flag::Flag;

fn parse_operand(string: &str) -> Option<String> {
    if string.is_empty() {
        return None;
    }
    if let Some(c) = string.strip_prefix('!') {
        return Some(format!("OperandType::Hidden({c})"));
    }
    Some(match string {
        "addr" => "OperandType::Address".into(),
        "dst" | "src" | "reg" => "OperandType::Register".into(),
        "#imm8" | "imm8" => "OperandType::Byte".into(),
        "#imm" | "#imm16" | "imm" | "imm16" => "OperandType::Word".into(),
        "rel" => "OperandType::Offset".into(),
        c => format!("OperandType::Const(\"{c}\")"),
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
        let opcode =
            u8::from_str_radix(parts.next().expect("Expected opcode"), 16).expect("Invalid opcode");
        let mnem = parts.next().expect("Expected mnemonic");
        if mnem.is_empty() || mnem.starts_with('#') {
            continue;
        }
        let op1 = parse_operand(parts.next().unwrap_or(""));
        let op2 = parse_operand(parts.next().unwrap_or(""));
        let op3 = parse_operand(parts.next().unwrap_or(""));
        let _bytes = parts.next();
        let _desc = parts.next();
        for cc in if mnem.contains("{cc}") {
            vec![
                "B", "BE", "EQ", "AE", "A", "NB", "NBE", "NEQ", "NAE", "NA", "L", "LE", "E", "G",
                "GE", "NL", "NLE", "NE", "NG", "NGE", "LT", "GE", "NLT", "NGE", "Z", "S", "C", "O",
                "NZ", "NS", "NC", "NO",
            ]
            .into_iter()
            .map(Some)
            .collect()
        } else {
            vec![None]
        } {
            for flag in if mnem.contains("{flag}") {
                vec!["ZF", "SF", "CF", "OF", "EIF", "HLT"]
                    .into_iter()
                    .map(Some)
                    .collect()
            } else {
                vec![None]
            } {
                result.push_str("  ");
                let mut mnem = mnem.to_string();
                let mut mnem_ops = String::new();
                if let Some(cc) = cc {
                    mnem = mnem.replace("{cc}", cc);
                    let cc_val = u8::from(ConditionCode::from_str(cc).unwrap());
                    mnem_ops.push_str(&format!("OperandType::Hidden({cc_val}), "));
                }
                if let Some(flag) = flag {
                    mnem = mnem.replace("{flag}", flag);
                    for flag_val in (1u16 << u8::from(Flag::from_str(flag).unwrap()))
                        .to_le_bytes()
                        .into_iter()
                    {
                        mnem_ops.push_str(&format!("OperandType::Hidden({flag_val}), "));
                    }
                }
                result.push_str(&format!("({opcode}, \"{mnem}\", &[{mnem_ops}"));
                if let (Some("OperandType::Register"), Some("OperandType::Register")) =
                    (op1.as_deref(), op2.as_deref())
                {
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
                    if let Some(op3) = &op3 {
                        result.push_str(op3);
                        result.push_str(", ");
                    }
                }
                result.push_str("]),\n");
            }
        }
        result.push_str("\n");
    }
    result.push_str("]");
    fs::write(dest_path, result).unwrap();
    println!("cargo:rerun-if-changed=instructions.tsv");
    println!("cargo:rerun-if-changed=build.rs");
}
