pub fn intcode_computer(contents: String, day: usize, input: isize) -> isize {
    let mut vec: Vec<isize> = contents.split(",").map(|num| num.parse().unwrap()).collect();
    let mut instruction_pointer = 0;
    let mut diagnostic_output = 0;
    while instruction_pointer < vec.len() {
        let (op, mode1, mode2, _mode3) = intcode_parser(vec[instruction_pointer], day);

        match op {
            99 => break,
            1 => {
                let val1 = if mode1 == 0 {vec[vec[instruction_pointer + 1] as usize]} else {vec[instruction_pointer + 1]};
                let val2 = if mode2 == 0 {vec[vec[instruction_pointer + 2] as usize]} else {vec[instruction_pointer + 2]};
                let val3 = vec[instruction_pointer + 3];
                vec[val3 as usize] = val1 + val2;
                instruction_pointer += 4;
            },
            2 => {
                let val1 = if mode1 == 0 {vec[vec[instruction_pointer + 1] as usize]} else {vec[instruction_pointer + 1]};
                let val2 = if mode2 == 0 {vec[vec[instruction_pointer + 2] as usize]} else {vec[instruction_pointer + 2]};
                let val3 = vec[instruction_pointer + 3];
                vec[val3 as usize] = val1 * val2;
                instruction_pointer += 4;
            },
            3 => {
                let val = vec[instruction_pointer + 1];
                vec[val as usize] = input; // input 1 or 5
                instruction_pointer += 2;
            },
            4 => {
                let val = if mode1 == 0 {vec[vec[instruction_pointer + 1] as usize]} else {vec[instruction_pointer + 1]};
                if val != 0 {
                    diagnostic_output = val; // output
                }
                instruction_pointer += 2;
            },
            5 => {
                let val1 = if mode1 == 0 {vec[vec[instruction_pointer + 1] as usize]} else {vec[instruction_pointer + 1]};
                let val2 = if mode2 == 0 {vec[vec[instruction_pointer + 2] as usize]} else {vec[instruction_pointer + 2]};
                instruction_pointer = if val1 != 0 {val2 as usize} else {instruction_pointer + 3};
            },
            6 => {
                let val1 = if mode1 == 0 {vec[vec[instruction_pointer + 1] as usize]} else {vec[instruction_pointer + 1]};
                let val2 = if mode2 == 0 {vec[vec[instruction_pointer + 2] as usize]} else {vec[instruction_pointer + 2]};
                instruction_pointer = if val1 == 0 {val2 as usize} else {instruction_pointer + 3};
            },
            7 => {
                let val1 = if mode1 == 0 {vec[vec[instruction_pointer + 1] as usize]} else {vec[instruction_pointer + 1]};
                let val2 = if mode2 == 0 {vec[vec[instruction_pointer + 2] as usize]} else {vec[instruction_pointer + 2]};
                let val3 = vec[instruction_pointer + 3];
                vec[val3 as usize] = if val1 < val2 {1} else {0};
                instruction_pointer += 4;
            },
            8 => {
                let val1 = if mode1 == 0 {vec[vec[instruction_pointer + 1] as usize]} else {vec[instruction_pointer + 1]};
                let val2 = if mode2 == 0 {vec[vec[instruction_pointer + 2] as usize]} else {vec[instruction_pointer + 2]};
                let val3 = vec[instruction_pointer + 3];
                vec[val3 as usize] = if val1 == val2 {1} else {0};
                instruction_pointer += 4;
            },
            _ => println!("something went wrong")
        }
    }
    return diagnostic_output;
}

fn intcode_parser(instruction: isize, day: usize) -> (usize, usize, usize, usize) {
    let mut instructions: Vec<char> = instruction.to_string().chars().map(|d| d).collect();
    instructions.reverse();
    let mut op: usize = 0;
    let mut mode1 : usize = 0;
    let mut mode2: usize = 0;
    let mode3: usize = 0;
    let mut cur_index = 0;
    while cur_index < instructions.len() {
        let ins = instructions[cur_index];
        if cur_index == 0 {
            if '9' == ins {
                op = 99;
            } else {
                op = ins.to_digit(10).unwrap() as usize;
            }
            if instructions.len() > 1 {
                cur_index += 1;    
            }
        } else if cur_index == 2 {
            mode1 = ins.to_digit(10).unwrap() as usize;
        } else if cur_index == 3 {
            mode2 = ins.to_digit(10).unwrap() as usize;
        }
        cur_index += 1;
    }
    if day != 5 {
        return (op, 0, 0, 0)
    }
    (op, mode1, mode2, mode3)
}