use assembler::Y86Assembler;
use clap::App;
use std::error::Error;
fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("Y86 Debugger")
        .version("1.0")
        .author("Tarik Eshaq")
        .about("My Y86 Assembler")
        .args_from_usage("<INPUT>              'Sets the ys file to assemble'")
        .get_matches();
    let file_name = matches
        .value_of("INPUT")
        .expect("Please input a file to debug")
        .to_string();

    let mut assembler = Y86Assembler::from_file(file_name.clone())?;
    assembler.save_file(file_name + ".mem")?;
    Ok(())
}
