pub fn day4a() {
    let mut possible_passwords: u32 = 0;
    let range = 109165..576723;

    for number in range {
        let digits: Vec<u32> = number
            .to_string()
            .chars()
            .map(|d| d.to_digit(10).unwrap())
            .collect();
        if is_decreasing(digits.clone()) {
            continue;
        }

        if !has_adjacent_digit(digits) {
            continue;
        }
        possible_passwords += 1;
    }
    println!("Result of day4a is {}", possible_passwords);
}

pub fn day4b() {
    let mut possible_passwords: u32 = 0;
    let range = 109165..576723;

    for number in range {
        let digits: Vec<u32> = number
            .to_string()
            .chars()
            .map(|d| d.to_digit(10).unwrap())
            .collect();
        if is_decreasing(digits.clone()) {
            continue;
        }

        if !has_adjacent_digit_of_two(digits) {
            continue;
        }
        possible_passwords += 1;
    }
    println!("Result of day4b is {}", possible_passwords);
}

fn is_decreasing(digits: Vec<u32>) -> bool {
    let mut prev: u32 = 0;

    for digit in digits {
        if digit < prev {
            return true;
        }
        prev = digit;
    }
    false
}

fn has_adjacent_digit(digits: Vec<u32>) -> bool {
    let mut prev: u32 = 0;

    for digit in digits {
        if digit == prev {
            return true;
        }
        prev = digit;
    }
    false
}

fn has_adjacent_digit_of_two(digits: Vec<u32>) -> bool {
    let mut adjacent_vec: Vec<u32> = Vec::new();

    for digit in digits {
        if !adjacent_vec.is_empty() && &digit != adjacent_vec.last().unwrap() {
            if adjacent_vec.len() == 2 {
                return true;
            }
            adjacent_vec.clear();
        }
        adjacent_vec.push(digit);
    }
    adjacent_vec.len() == 2
}
