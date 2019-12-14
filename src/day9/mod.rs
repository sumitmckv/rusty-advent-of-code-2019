use crate::lib::Computer;
use std::fs;
use std::path::PathBuf;

pub fn day9() -> std::io::Result<()> {
    let input_file = PathBuf::from("./src/day9/input.txt").canonicalize()?;
    let input_txt = fs::read_to_string(input_file).unwrap();
    let mut computer1 = Computer::new();
    let mut computer2 = Computer::new();
    computer1.init(input_txt.trim());
    computer2.init(input_txt.trim());
    computer1.write(1);
    computer2.write(2);
    computer1.execute();
    computer2.execute();
    println!("Reasult of day9a is {}", computer1.get_output());
    println!("Reasult of day9a is {}", computer2.get_output());
    Ok(())
}
