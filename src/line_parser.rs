use std::error::Error;
use crate::instruction;
pub fn parse(line: &String) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut split_line = line.split(" ");
    let instr = instruction::Instruction::new(&split_line.next().unwrap().to_string())?;
    instr.parse(line)
}