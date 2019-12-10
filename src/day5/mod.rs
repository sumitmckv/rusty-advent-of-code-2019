use crate::lib::Computer;
use std::fs;
use std::path::PathBuf;

pub fn day5a() -> std::io::Result<()> {
    let input_file = PathBuf::from("./src/day5/input.txt").canonicalize()?;
    let input_txt = fs::read_to_string(input_file).unwrap();
    let mut computer = Computer::new();
    computer.init(input_txt.trim());
    computer.write(1);
    computer.execute();
    println!("Result of day5a is {}", computer.get_output());
    Ok(())
}

pub fn day5b() -> std::io::Result<()> {
    let input_file = PathBuf::from("./src/day5/input.txt").canonicalize()?;
    let input_txt = fs::read_to_string(input_file).unwrap();
    let mut computer = Computer::new();
    computer.init(input_txt.trim());
    computer.write(5);
    computer.execute();
    println!("Result of day5b is {}", computer.get_output());
    Ok(())
}
