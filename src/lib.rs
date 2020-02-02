use std::collections::BTreeMap;
use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::u64;
use std::collections::HashMap;
mod parser;
mod number_parser;
use parser::ICode;

pub struct Y86Assembler {
    bytes: Vec<u8>
}

impl Y86Assembler {
    pub fn from_file(file_name: String) -> Result<Self, Box<dyn Error>> {
        let lines_iter = read_lines(file_name)?;
        let lines: Vec<String> = lines_iter.map(|val| val.unwrap()).collect();
        let mut positions: BTreeMap<u64, Vec<u8>> = BTreeMap::new();
        get_positions(&mut positions, &lines)?;
        Ok(Y86Assembler{
            bytes: merge_position(&positions)
        })
    }

    pub fn to_file(&mut self, file_name: String) -> Result<(), Box<dyn Error>> {
        let mut file = File::create(file_name)?;
        file.write_all(&self.bytes)?;
        Ok(())
    }

    pub fn print(&self) {
        self.bytes.iter().for_each(|b| print!("{:02x}", b));
        println!("");
    }
}

fn merge_position(positions: &BTreeMap<u64, Vec<u8>>) -> Vec<u8> {
    let mut iter = positions.iter();
    let mut res = vec![];
    while let Some((key, val)) = iter.next() {
        while res.len() < *key as usize {
            res.push(0);
        }
        val.iter().for_each(|&b| res.push(b));
    }
    res
}

fn get_positions(
    positions: &mut BTreeMap<u64, Vec<u8>>,
    lines: &Vec<String>,
) -> Result<(), Box<dyn Error>> {
    let mut curr_position = 0;
    let trimmed = lines.iter().map(|line| trim_line(&line)).collect();
    let mapping: HashMap<&str, u64> = map_labels(&trimmed)?;
    let val: Result<(), Box<dyn Error>> = trimmed.iter().map(|line| apply_mapping(&mapping, &line)).try_for_each(|line| {
        if line.starts_with(".pos") {
            let position: u64 = number_parser::parse_num(&line[5..])?;
            positions.insert(position, vec![]);
            curr_position = position;
        } else {
            let curr_vec = positions.entry(curr_position).or_insert(vec![]);
            curr_vec.append(&mut convert_line(&line)?);
        }
        Ok(())
    });
    val
}

fn trim_line(line: &String) -> String {
    let mut res = line.trim().to_string();
    if res.contains("#") {
        res = res[..res.find("#").unwrap()].to_string();
    }
    res.replace("$", "").to_string()
}

fn apply_mapping(mapping: &HashMap<&str, u64>, line: &String) -> String {
    let mut res = String::new();
    if line.contains(":") {
        res.push_str(line[line.find(":").unwrap() + 1 ..].trim());
    } else {
        res = line.clone();
    }
    mapping.iter().for_each(|(key, val)| {
        let mut expected = " ".to_string();
        expected.push_str(key);
        if res.contains(&expected) {
            res = res.replace(key, &format!("0x{:x}", val)).to_string();
        }
    });
    res
}

fn instr_size(line: &String) -> Result<u64, Box<dyn Error>> {
    let mut split = line.split(" ");
    let instr = split.next().unwrap();
    let val = match parser::get_icode_from_string(instr)? {
        ICode::IIRMOVQ | ICode::IRMMOVQ | ICode::IMRMOVQ => 10,
        ICode::IRRMVXX | ICode::IOPQ | ICode::IPOPQ | ICode::IPUSHQ => 2,
        ICode::IJXX | ICode::ICALL => 9,
        ICode::IHALT | ICode::INOP | ICode::IRET => 1,
        _ => 0
    };
    Ok(val)
}

fn map_labels(lines: &Vec<String>) -> Result<HashMap<&str, u64>, Box<dyn Error>> {
    let mut res: HashMap<&str, u64> = HashMap::new();
    let mut curr_addr = 0;
    let val: Result<(), Box<dyn Error>> = lines.iter().try_for_each(|line| {
        if line.starts_with(".pos") {
            let position: u64 = number_parser::parse_num(&line[5..])?;
            curr_addr = position;
        } else {
            if line.contains(":") {
                let mut split = line.split(":");
                res.insert(split.next().unwrap().trim(), curr_addr);
            }
            if line.contains(".quad") {
                curr_addr += 8;
            } else if line.len() != 0 {
                let mut line = line.clone();
                if line.contains(":") {
                    line = line[line.find(":").unwrap() + 1..].trim().to_string();
                }
                curr_addr += instr_size(&line)?;
            }
        }
        Ok(())
    });
    val?;
    Ok(res)
}

fn convert_line(line: &String) -> Result<Vec<u8>, Box<dyn Error>> {
    if line.trim().len() == 0 {
        return Ok(vec![]);
    }
    parser::parse(line)
}

fn read_lines(file_name: String) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(file_name)?;
    Ok(io::BufReader::new(file).lines())
}

// Go over each .pos, starting form there, pump values into a hashmap
// Sort the map by key, then add values, with 000 between to the end result.
