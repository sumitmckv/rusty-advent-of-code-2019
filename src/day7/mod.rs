use crate::lib::Computer;
use permutohedron::Heap;
use std::fs;
use std::path::PathBuf;

pub fn day7a() {
    let input_file = PathBuf::from("./src/day7/input.txt")
        .canonicalize()
        .unwrap();
    let input_txt = fs::read_to_string(input_file).unwrap();
    let permutations = Heap::new(&mut [0, 1, 2, 3, 4]).collect::<Vec<_>>();
    let mut max_thruster = 0;
    for phase_settings in permutations {
        let mut output = 0;
        for phase in phase_settings.iter() {
            let mut computer = Computer::new();
            computer.init(input_txt.trim());
            computer.write(phase.to_owned());
            computer.write(output);
            computer.execute();
            output = computer.get_output();
        }
        if output > max_thruster {
            max_thruster = output
        }
    }
    println!("Result of day7a {}", max_thruster);
}

pub fn day7b() {
    let input_file = PathBuf::from("./src/day7/input.txt")
        .canonicalize()
        .unwrap();
    let input_txt = fs::read_to_string(input_file).unwrap();
    let permutations = Heap::new(&mut [5, 6, 7, 8, 9]).collect::<Vec<_>>();
    let mut max_thruster = 0;

    for phase_settings in permutations {
        let output = feedback_loop(input_txt.trim(), phase_settings.to_vec());
        if output > max_thruster {
            max_thruster = output
        }
    }
    println!("Result of day7b {}", max_thruster);
}

fn feedback_loop(prog_txt: &str, phase_settings: Vec<isize>) -> isize {
    let mut amp1 = Computer::new();
    amp1.init(prog_txt);
    amp1.write(phase_settings[0]);

    let mut amp2 = Computer::new();
    amp2.init(prog_txt);
    amp2.write(phase_settings[1]);

    let mut amp3 = Computer::new();
    amp3.init(prog_txt);
    amp3.write(phase_settings[2]);

    let mut amp4 = Computer::new();
    amp4.init(prog_txt);
    amp4.write(phase_settings[3]);

    let mut amp5 = Computer::new();
    amp5.init(prog_txt);
    amp5.write(phase_settings[4]);

    let mut output = 0;

    while !amp5.is_halted() {
        output = process(&mut amp1, output);
        output = process(&mut amp2, output);
        output = process(&mut amp3, output);
        output = process(&mut amp4, output);
        output = process(&mut amp5, output);
    }

    amp5.get_output()
}

fn process(amp: &mut Computer, input: isize) -> isize {
    amp.write(input);
    amp.execute();
    amp.get_output()
}
