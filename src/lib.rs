use std::error::Error;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::collections::BTreeMap;
use std::u64;
mod line_parser;
mod instruction;
pub fn dump(file_name: String) -> Result<Vec<u8>, Box<dyn Error>> {
    // Strategy:
    // Convert all labels to values
    // Line by line
    // convert each line to bytes
        //Read instruction name
        // For each instruction call coresponding function
            // If invalid propegate error all the way up,
            // If valid, compose appropriate little endian bytes and return
    
    // Handle .pos declarations 
    // Handle
    // Add bytes to res
    let lines_iter = read_lines(file_name)?;
    let lines: Vec<String> = lines_iter.map(|val| val.unwrap()).collect();
    let mut positions: BTreeMap<u64, Vec<u8>> = BTreeMap::new();
    get_positions(&mut positions, &lines)?;
    Ok(vec![])
}

fn get_positions(positions: &mut BTreeMap<u64, Vec<u8>>, lines: &Vec<String>) -> Result<(), Box<dyn Error>> {
    let mut curr_position = 0;
    let val: Result<(), Box<dyn Error>> = lines.iter().try_for_each(|line| {
        if line.starts_with(".pos") {
            let position: u64 = u64::from_str_radix(&line[7..], 16)?;
            positions.insert(position, vec![]);
            // Add 00s between if needed
            curr_position = position;
        } else {
            let curr_vec = positions.entry(curr_position).or_insert(vec![]);
            curr_vec.append(&mut convert_line(&line)?);
        }
        Ok(())
    });
    Ok(())
}

fn convert_line(line: &String) -> Result<Vec<u8>, Box<dyn Error>> {
    if line.trim().len() == 0 {
        return Ok(vec![]);
    }
    line_parser::parse(line)
}

fn read_lines(file_name: String) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(file_name)?;
    Ok(io::BufReader::new(file).lines())
}

// Go over each .pos, starting form there, pump values into a hashmap
// Sort the map by key, then add values, with 000 between to the end result.
