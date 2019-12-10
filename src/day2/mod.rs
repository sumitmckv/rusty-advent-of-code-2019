use crate::lib::{Computer, Mode};
use std::fs;
use std::path::PathBuf;

pub fn day2a() -> std::io::Result<()> {
    let input_file = PathBuf::from("./src/day2/input.txt").canonicalize()?;
    let input_txt = fs::read_to_string(input_file).unwrap();
    let mut computer = Computer::new();
    computer.init(input_txt.trim());
    computer.set(1, 12, Mode::Immediate);
    computer.set(2, 2, Mode::Immediate);
    computer.execute();
    println!("Result of day2a is {}", computer.read(0));
    Ok(())
}

pub fn day2b() -> std::io::Result<()> {
    let input_file = PathBuf::from("./src/day2/input.txt").canonicalize()?;
    let input_txt = fs::read_to_string(input_file).unwrap();
    let target: isize = 19690720;
    let mut terminate: bool = false;
    for noun in 0..99 {
        for verb in 0..99 {
            let mut computer = Computer::new();
            computer.init(input_txt.trim());
            computer.set(1, noun, Mode::Immediate);
            computer.set(2, verb, Mode::Immediate);
            computer.execute();
            if target == computer.read(0) {
                println!("Result of day2b is {}", 100 * noun + verb);
                terminate = true;
                break;
            }
        }
        if terminate == true {
            break;
        }
    }
    Ok(())
}
