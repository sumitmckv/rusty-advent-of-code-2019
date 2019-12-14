use std::collections::{HashMap, VecDeque};
use std::convert::{TryFrom, TryInto};

#[derive(Debug)]
pub enum Mode {
    Position,
    Immediate,
    Relative,
}

#[derive(Debug)]
pub enum Intcode {
    Add(Mode, Mode, Mode),
    Mult(Mode, Mode, Mode),
    In(Mode),
    Out(Mode),
    Jit(Mode, Mode),
    Jif(Mode, Mode),
    Lt(Mode, Mode, Mode),
    Eq(Mode, Mode, Mode),
    Rel(Mode),
    Halt,
}

#[derive(Debug)]
pub enum ErrorIntcode {
    InvalidOpcode,
    InvalidMode,
}

impl TryFrom<isize> for Mode {
    type Error = ErrorIntcode;

    fn try_from(value: isize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Mode::Position),
            1 => Ok(Mode::Immediate),
            2 => Ok(Mode::Relative),
            _ => Err(ErrorIntcode::InvalidMode),
        }
    }
}

impl TryFrom<isize> for Intcode {
    type Error = ErrorIntcode;

    fn try_from(value: isize) -> Result<Self, Self::Error> {
        let m1: Mode = ((value / 100) % 10).try_into()?;
        let m2: Mode = ((value / 1000) % 10).try_into()?;
        let m3: Mode = ((value / 10000) % 10).try_into()?;

        match value % 100 {
            1 => Ok(Intcode::Add(m1, m2, m3)),
            2 => Ok(Intcode::Mult(m1, m2, m3)),
            3 => Ok(Intcode::In(m1)),
            4 => Ok(Intcode::Out(m1)),
            5 => Ok(Intcode::Jit(m1, m2)),
            6 => Ok(Intcode::Jif(m1, m2)),
            7 => Ok(Intcode::Lt(m1, m2, m3)),
            8 => Ok(Intcode::Eq(m1, m2, m3)),
            9 => Ok(Intcode::Rel(m1)),
            99 => Ok(Intcode::Halt),
            _ => Err(ErrorIntcode::InvalidOpcode),
        }
    }
}

#[derive(Debug)]
pub struct Computer {
    memory: HashMap<usize, isize>, // intcode instructions
    ptr: usize,                    // instruction pointer
    inputs: VecDeque<isize>,       // input buffer
    output: isize,                 // output buffer
    halted: bool,                  // indicates if program halted
    rel_base: usize,               // relative base index for Relative Mode
}

impl Computer {
    pub fn new() -> Computer {
        Computer {
            memory: HashMap::new(),
            ptr: 0,
            inputs: VecDeque::new(),
            output: 0,
            halted: false,
            rel_base: 0,
        }
    }

    pub fn init(&mut self, input_txt: &str) {
        let arr: Vec<isize> = input_txt
            .split(",")
            .map(|a| a.parse::<isize>().unwrap())
            .collect();
        for loc in 0..arr.len() {
            self.memory.insert(loc, arr[loc]);
        }
    }

    pub fn get(&self, addr: usize, m: Mode) -> isize {
        match m {
            Mode::Position => self.read(self.read(addr) as usize),
            Mode::Immediate => self.read(addr),
            Mode::Relative => self.read(self.rel_base_val(addr) as usize),
        }
    }

    pub fn set(&mut self, addr: usize, val: isize, m: Mode) {
        match m {
            Mode::Position => *self.memory.entry(self.read(addr) as usize).or_insert(val) = val,
            Mode::Immediate => *self.memory.entry(addr).or_insert(val) = val,
            Mode::Relative => {
                let real_addr = self.rel_base_val(addr) as usize;
                *self.memory.entry(real_addr).or_insert(val) = val
            }
        }
    }

    pub fn read(&self, addr: usize) -> isize {
        match self.memory.get(&addr) {
            Some(val) => *val,
            None => 0,
        }
    }

    pub fn write(&mut self, input: isize) {
        self.inputs.push_front(input);
    }

    pub fn get_output(&self) -> isize {
        self.output
    }

    pub fn is_halted(&self) -> bool {
        self.halted
    }

    fn rel_base_val(&self, addr: usize) -> isize {
        self.rel_base as isize + self.read(addr)
    }

    pub fn execute(&mut self) {
        loop {
            let op: Intcode = self.read(self.ptr).try_into().unwrap();
            match op {
                Intcode::Add(m1, m2, m3) => {
                    let (p_a, p_b) = (self.get(self.ptr + 1, m1), self.get(self.ptr + 2, m2));
                    self.set(self.ptr + 3, p_a + p_b, m3);
                    self.ptr += 4;
                }
                Intcode::Mult(m1, m2, m3) => {
                    let (p_a, p_b) = (self.get(self.ptr + 1, m1), self.get(self.ptr + 2, m2));
                    self.set(self.ptr + 3, p_a * p_b, m3);
                    self.ptr += 4;
                }
                Intcode::In(m1) => {
                    match self.inputs.pop_back() {
                        Some(val) => self.set(self.ptr + 1, val, m1),
                        None => {
                            self.halted = false;
                            break;
                        }
                    }
                    self.ptr += 2;
                }
                Intcode::Out(m1) => {
                    self.output = self.get(self.ptr + 1, m1);
                    self.ptr += 2;
                }
                Intcode::Jit(m1, m2) => {
                    let (p_a, p_b) = (self.get(self.ptr + 1, m1), self.get(self.ptr + 2, m2));
                    if p_a != 0 {
                        self.ptr = p_b as usize;
                    } else {
                        self.ptr += 3;
                    }
                }
                Intcode::Jif(m1, m2) => {
                    let (p_a, p_b) = (self.get(self.ptr + 1, m1), self.get(self.ptr + 2, m2));
                    if p_a == 0 {
                        self.ptr = p_b as usize;
                    } else {
                        self.ptr += 3;
                    }
                }
                Intcode::Lt(m1, m2, m3) => {
                    let (p_a, p_b) = (self.get(self.ptr + 1, m1), self.get(self.ptr + 2, m2));
                    self.set(self.ptr + 3, (p_a < p_b) as isize, m3);
                    self.ptr += 4;
                }
                Intcode::Eq(m1, m2, m3) => {
                    let (p_a, p_b) = (self.get(self.ptr + 1, m1), self.get(self.ptr + 2, m2));
                    self.set(self.ptr + 3, (p_a == p_b) as isize, m3);
                    self.ptr += 4;
                }
                Intcode::Rel(m1) => {
                    self.rel_base = (self.rel_base as isize + self.get(self.ptr + 1, m1)) as usize;
                    self.ptr += 2;
                }
                Intcode::Halt => {
                    self.halted = true;
                    break;
                }
            }
        }
    }
}
