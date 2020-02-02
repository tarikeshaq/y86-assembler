use clap::App;
use assembler::Y86Assembler;
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
    
    let mut assembler = Y86Assembler::from_file(file_name.clone())?;
    assembler.to_file(file_name + ".mem")?;
    assembler.print();
    Ok(())
}
