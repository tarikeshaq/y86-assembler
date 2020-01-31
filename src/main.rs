
use memdump::dump;
use clap::App;
use std::error::Error;
fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("Y86 Debugger")
        .version("1.0")
        .author("Tarik Eshaq")
        .about("My Y86 Debugger!")
        .args_from_usage("<INPUT>              'Sets the mem file to debug'")
        .get_matches();
    let file_name = matches
        .value_of("INPUT")
        .expect("Please input a file to debug")
        .to_string();
    Ok(dump(file_name)?.iter().for_each(|val| print!("{:02}", val)))
}
