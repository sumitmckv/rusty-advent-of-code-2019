#[rustfmt::skip]
pub fn intcode_computer(mut instructions: Vec<isize>, day: usize, input: isize) -> isize {
    let mut instruction_pointer = 0;
    let mut diagnostic_output = 0;
    while instruction_pointer < instructions.len() - 2 {
        let (op, mode1, mode2, _mode3) = intcode_parser(instructions[instruction_pointer], day);
        let param1 = instructions[instruction_pointer + 1];
        let param2 = instructions[instruction_pointer + 2];
        let param3 = instructions[instruction_pointer + 3];
        match op {
            99 => break,
            1 => { // addition
                let val1 = if mode1 == 0 { instructions[param1 as usize] } else { param1 };
                let val2 = if mode2 == 0 { instructions[param2 as usize] } else { param2 };
                instructions[param3 as usize] = val1 + val2;
                instruction_pointer += 4;
            }
            2 => { // multiplication
                let val1 = if mode1 == 0 { instructions[param1 as usize] } else { param1 };
                let val2 = if mode2 == 0 { instructions[param2 as usize] } else { param2 };
                instructions[param3 as usize] = val1 * val2;
                instruction_pointer += 4;
            }
            3 => { // input
                instructions[param1 as usize] = input;
                instruction_pointer += 2;
            }
            4 => { // output
                let val = if mode1 == 0 { instructions[param1 as usize] } else { param1 };
                if val != 0 {
                    diagnostic_output = val;
                }
                instruction_pointer += 2;
            }
            5 => { // jump-if-true
                let val1 = if mode1 == 0 { instructions[param1 as usize] } else { param1 };
                let val2 = if mode2 == 0 { instructions[param2 as usize] } else { param2 };
                instruction_pointer = if val1 != 0 { val2 as usize } else { instruction_pointer + 3 };
            }
            6 => { // jump-if-false
                let val1 = if mode1 == 0 { instructions[param1 as usize] } else { param1 };
                let val2 = if mode2 == 0 { instructions[param2 as usize] } else { param2 };
                instruction_pointer = if val1 == 0 { val2 as usize } else { instruction_pointer + 3 };
            }
            7 => { // less than
                let val1 = if mode1 == 0 { instructions[param1 as usize] } else { param1 };
                let val2 = if mode2 == 0 { instructions[param2 as usize] } else { param2 };
                instructions[param3 as usize] = if val1 < val2 { 1 } else { 0 };
                instruction_pointer += 4;
            }
            8 => { // equals
                let val1 = if mode1 == 0 { instructions[param1 as usize] } else { param1 };
                let val2 = if mode2 == 0 { instructions[param2 as usize] } else { param2 };
                instructions[param3 as usize] = if val1 == val2 { 1 } else { 0 };
                instruction_pointer += 4;
            }
            _ => println!("something went wrong"),
        }
    }
    if day == 2 {
        return instructions[0];
    }
    return diagnostic_output;
}

fn intcode_parser(instruction: isize, day: usize) -> (usize, usize, usize, usize) {
    let mut instructions: Vec<char> = instruction.to_string().chars().map(|d| d).collect();
    instructions.reverse();
    let mut op: usize = 0;
    let mut mode1: usize = 0;
    let mut mode2: usize = 0;
    let mode3: usize = 0;
    let mut cur_index = 0;
    while cur_index < instructions.len() {
        let ins = instructions[cur_index];
        match cur_index {
            0 => {
                op = ins.to_digit(10).unwrap() as usize;
                op = if 9 == op { 99 } else { op };
                cur_index = if instructions.len() > 1 { 1 } else { 0 };
            }
            2 => mode1 = ins.to_digit(10).unwrap() as usize,
            3 => mode2 = ins.to_digit(10).unwrap() as usize,
            _ => println!("Unexpected index received"),
        }
        cur_index += 1;
    }
    if day == 2 {
        return (op, 0, 0, 0);
    }
    (op, mode1, mode2, mode3)
}
