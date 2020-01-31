use std::fmt::{self,Display, Formatter};

enum InstructionType {
    IRMOVQ,
    RRMOVXX,
    MRMOVQ,
    RMMOVQ,
    JXX,
    CALL,
    RET,
    PUSHQ,
    POPQ,
    OPQ,
    HALT,
    NOP
}

pub struct Instruction {
    instruction_string: String,
    instruction_type: InstructionType
}

#[derive(Debug)]
struct InvalidInstructionError;

impl std::error::Error for InvalidInstructionError {
}

impl Display for InvalidInstructionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid instruction")
    }
}

impl Instruction {
    pub fn new(instr: &String) -> Result<Self, Box<dyn std::error::Error>> {
        let instruction_type = match &instr[..] {
            "irmovq" => InstructionType::IRMOVQ,
            "rrmovq" | "cmovle" | "cmovl" | "cmove" | "cmovne" | "cmovg" | "cmovge" => InstructionType::RRMOVXX,
            "mrmovq" => InstructionType::MRMOVQ,
            "rmmovq" => InstructionType::RMMOVQ,
            "jmp" | "jle" | "jl" | "je" | "jne" | "jg" | "jge" => InstructionType::JXX,
            "call" => InstructionType::CALL,
            "ret" => InstructionType::RET,
            "pushq" => InstructionType::PUSHQ,
            "popq" => InstructionType::POPQ,
            "addq" | "subq" | "divq" | "multq" | "modq" | "xorq" | "andq" => InstructionType::OPQ,
            "halt" => InstructionType::HALT,
            "nop" => InstructionType::NOP,
            _ => {
                let boxed: Box<InvalidInstructionError> = InvalidInstructionError.into();
                Err(boxed)?
            }
        };
        Ok(Instruction {
            instruction_string: instr.clone(),
            instruction_type
        })
    }

    pub fn parse(&self, line: &String) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        match self.instruction_type {
            InstructionType::IRMOVQ => parse_irmovq(line),
            InstructionType::RRMOVXX => parse_rrmovxx(line),
            InstructionType::MRMOVQ => parse_mrmovq(line),
            InstructionType::RMMOVQ => parse_rmmovq(line),
            InstructionType::JXX => parse_jxx(line),
            InstructionType::CALL => parse_call(line),
            InstructionType::RET => parse_ret(line),
            InstructionType::PUSHQ => parse_pushq(line),
            InstructionType::POPQ => parse_popq(line),
            InstructionType::OPQ => parse_opq(line),
            InstructionType::HALT => parse_halt(line),
            InstructionType::NOP => parse_nop(line),

        }
    }
}

fn form_byte(first: u8, second: u8) -> u8 {
    ((first << 4) & 0xF0) | (second & 0x0F)
}

fn get_immediate(value: &str) -> u64 {
    unimplemented!();
}

fn get_register(value: &str) -> u8 {
    unimplemented!();
}

fn push_le(vec: &mut Vec<u8>, val: u64) {
    unimplemented!();
}

fn parse_irmovq(line: &String) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
   println!("Adding irmovq");
   let mut res = Vec::new();
   let b: u8 = form_byte(3, 0);
   res.push(b);
   let mut split = line.split(",");
   let val_c = get_immediate(split.next().unwrap());
   let reg = get_register(split.next().unwrap());
   let b: u8 = form_byte(0x0F, reg);
   res.push(b);
   push_le(&mut res, val_c);
   Ok(res)
}

fn parse_rrmovxx(line: &String) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    unimplemented!("Not implemented")
}
fn parse_mrmovq(line: &String) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    unimplemented!("Not implemented")
}
fn parse_rmmovq(line: &String) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    unimplemented!("Not implemented")
}
fn parse_jxx(line: &String) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    unimplemented!("Not implemented")
}
fn parse_call(line: &String) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    unimplemented!("Not implemented")
}
fn parse_ret(line: &String) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    unimplemented!("Not implemented")
}
fn parse_pushq(line: &String) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    unimplemented!("Not implemented")
}
fn parse_popq(line: &String) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    unimplemented!("Not implemented")
}
fn parse_opq(line: &String) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    unimplemented!("Not implemented")
}
fn parse_halt(line: &String) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    unimplemented!("Not implemented")
}
fn parse_nop(line: &String) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    unimplemented!("Not implemented")
}