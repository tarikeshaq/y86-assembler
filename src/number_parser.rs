use std::error::Error;

pub fn parse_num(value: &str) -> Result<u64, Box<dyn Error>> {
    let val;
    if value.trim().starts_with("0x") {
        val = u64::from_str_radix(&value[2..], 16)?;
    } else {
        val = u64::from_str_radix(value, 10)?;
    }
    Ok(val)
}