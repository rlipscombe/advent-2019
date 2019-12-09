pub struct Interpreter {
    memory: Vec<i64>,
    ip: usize,
    bp: usize,
}

#[derive(Debug)]
enum OpCode {
    Add, Mul, In, Out, Jt, Jf, Eq, Lt, AdjBp, Halt
}
#[derive(Debug)]
enum Mode {
    Pos, Imm, Rel
}
type Instr = (OpCode, Mode, Mode);

impl Interpreter {
    pub fn from_source(source: String) -> Interpreter {
        let bytes = source
            .trim_end()
            .split(",")
            .map(|s| s.parse::<i64>().unwrap())
            .collect();
        Interpreter::new(bytes)
    }

    pub fn new(bytes: Vec<i64>) -> Interpreter {
        Interpreter {
            memory: bytes,
            ip: 0,
            bp: 0,
        }
    }

    pub fn run<I, O>(&mut self, mut input: I, mut output: O)
    where
        I: FnMut() -> Option<i64>,
        O: FnMut(i64),
    {
        loop {
            let instr = self.read_instr(self.ip);
            println!("{}: {:?}", self.ip, instr);
            match instr {
                (OpCode::Add, lmode, rmode) => {
                    let lhs = self.read_as(self.ip + 1, lmode);
                    let rhs = self.read_as(self.ip + 2, rmode);
                    let trg = self.read(self.ip + 3) as usize;
                    let result = lhs + rhs;
                    //print_instr(self.ip, "+", lmode, lhs, rmode, rhs, trg, result);
                    self.write(trg, result);
                    self.ip += 4;
                }
                (OpCode::Mul, lmode, rmode) => {
                    let lhs = self.read_as(self.ip + 1, lmode);
                    let rhs = self.read_as(self.ip + 2, rmode);
                    let trg = self.read(self.ip + 3) as usize;
                    let result = lhs * rhs;
                    //print_instr(self.ip, "*", lmode, lhs, rmode, rhs, trg, result);
                    self.write(trg, result);
                    self.ip += 4;
                }
                (OpCode::In, _, _) => {
                    let p = self.read(self.ip + 1) as usize;
                    let value = input().unwrap();
                    println!(
                        "{}: [{}] <- in  ; {}",
                        self.ip,
                        self.read(self.ip + 1),
                        value
                    );
                    self.write(p, value);
                    self.ip += 2;
                }
                (OpCode::Out, mode, _) => {
                    let value = self.read_as(self.ip + 1, mode);
                    println!("{}: out [{}] = {}", self.ip, self.read(self.ip + 1), value);
                    output(value);
                    self.ip += 2;
                }
                (OpCode::Jt, lmode, rmode) => {
                    let val = self.read_as(self.ip + 1, lmode);
                    let dst = self.read_as(self.ip + 2, rmode) as usize;
                    println!(
                        "{}: jt [{}] {}",
                        self.ip,
                        self.read(self.ip + 1),
                        self.read(self.ip + 2)
                    );
                    if val != 0 {
                        self.ip = dst;
                    } else {
                        self.ip += 3;
                    }
                }
                (OpCode::Jf, lmode, rmode) =>{
                    let val = self.read_as(self.ip + 1, lmode);
                    let dst = self.read_as(self.ip + 2, rmode) as usize;
                    println!(
                        "{}: jf [{}] [{}]",
                        self.ip,
                        self.read(self.ip + 1),
                        self.read(self.ip + 2)
                    );
                    if val == 0 {
                        self.ip = dst;
                    } else {
                        self.ip += 3;
                    }
                }
                (OpCode::Eq, lmode, rmode) => {
                    let lhs = self.read_as(self.ip + 1, lmode);
                    let rhs = self.read_as(self.ip + 2, rmode);
                    let trg_p = self.read(self.ip + 3) as usize;
                    let result = lhs == rhs;
                    println!(
                        "{}: [{}] <- [{}] eq [{}]  ; {} == {} = {}",
                        self.ip,
                        self.read(self.ip + 3),
                        self.read(self.ip + 1),
                        self.read(self.ip + 2),
                        lhs,
                        rhs,
                        result
                    );
                    self.write(trg_p, result as i64);
                    self.ip += 4;
                }
                (OpCode::Lt, lmode, rmode) => {
                    let lhs = self.read_as(self.ip + 1, lmode);
                    let rhs = self.read_as(self.ip + 2, rmode);
                    let trg_p = self.read(self.ip + 3) as usize;
                    let result = lhs < rhs;
                    println!(
                        "{}: [{}] <- [{}] lt [{}]  ; {} lt {} = {}",
                        self.ip,
                        self.read(self.ip + 3),
                        self.read(self.ip + 1),
                        self.read(self.ip + 2),
                        lhs,
                        rhs,
                        result
                    );
                    self.write(trg_p, result as i64);
                    self.ip += 4;
                }
                (OpCode::AdjBp, mode, _) => {
                    let by = self.read_as(self.ip + 1, mode) as usize;
                    println!("{}: adjbp {}", self.ip, by);
                    self.bp += by;
                    self.ip += 2;
                }
                (OpCode::Halt, _, _) => {
                    println!("{}: halt", self.ip);
                    return;
                }
            }
        }
    }

    fn read(&self, p: usize) -> i64 {
        if self.memory.len() < p {
            0
        } else {
            self.memory[p]
        }
    }

    fn read_instr(&self, p: usize) -> Instr {
        if self.memory.len() < p {
            (OpCode::Halt, Mode::Pos, Mode::Pos)
        } else {
            let instr = self.memory[p];
            let op = instr % 100;
            let lmode = (instr / 100) % 10;
            let rmode = instr / 1000; 
            (to_opcode(op), to_mode(lmode), to_mode(rmode))
        }
    }

    fn read_as(&self, p: usize, mode: Mode) -> i64 {
        match mode {
            Mode::Imm => self.read(p),
            Mode::Pos => self.read(self.read(p) as usize),
            Mode::Rel => self.read(self.bp + self.read(p) as usize)
        }
    }

    fn write(&mut self, p: usize, v: i64) {
        const INVALID_MEMORY: i64 = 99; // Halt
        if p > self.memory.len() {
            self.memory.resize(p + 1, INVALID_MEMORY);
        }
        self.memory[p] = v;
    }
}

fn to_opcode(op: i64) -> OpCode {
    match op {
        1 => OpCode::Add,
        2 => OpCode::Mul,
        3 => OpCode::In, 
        4 => OpCode::Out,
        5 => OpCode::Jt,
        6 => OpCode::Jf,
        7 => OpCode::Eq,
        8 => OpCode::Lt,
        9 => OpCode::AdjBp,
        99 => OpCode::Halt,
        _ => {
            panic!("Invalid opcode {}", op);
        }
    }
}

fn to_mode(mode: i64) -> Mode {
    match mode {
        0 => Mode::Pos,
        1 => Mode::Imm,
        2 => Mode::Rel,
        _ => {
            panic!("Invalid mode {}", mode);
        }
    }
}

#[test]
fn test_add_pp() {
    let mut computer = Interpreter::from_source("1,7,8,0,4,0,99,-12,12".to_string());
    let mut outputs = vec![];
    computer.run(|| None, |o| outputs.push(o));
    assert_eq!(vec![0], outputs);
}

#[test]
fn test_add_ip() {
    let mut computer = Interpreter::from_source("101,-12,8,0,4,0,99,0,12".to_string());
    let mut outputs = vec![];
    computer.run(|| None, |o| outputs.push(o));
    assert_eq!(vec![0], outputs);
}

#[test]
fn test_add_pi() {
    let mut computer = Interpreter::from_source("1001,8,-12,0,4,0,99,0,12".to_string());
    let mut outputs = vec![];
    computer.run(|| None, |o| outputs.push(o));
    assert_eq!(vec![0], outputs);
}

#[test]
fn test_add_ii() {
    let mut computer = Interpreter::from_source("1101,123,-123,0,4,0,99".to_string());
    let mut outputs = vec![];
    computer.run(|| None, |o| outputs.push(o));
    assert_eq!(vec![0], outputs);
}

#[test]
fn test_eq_8() {
    let mut computer = Interpreter::from_source("3,9,8,9,10,9,4,9,99,-1,8".to_string());
    let mut outputs = vec![];
    computer.run(|| Some(8), |o| outputs.push(o));
    assert_eq!(vec![1], outputs);
}

#[test]
fn test_neq_8() {
    let mut computer = Interpreter::from_source("3,9,8,9,10,9,4,9,99,-1,8".to_string());
    let mut outputs = vec![];
    computer.run(|| Some(-8), |o| outputs.push(o));
    assert_eq!(vec![0], outputs);
}

#[test]
fn test_lt_8() {
    let mut computer = Interpreter::from_source("3,9,7,9,10,9,4,9,99,-1,8".to_string());
    let mut outputs = vec![];
    computer.run(|| Some(7), |o| outputs.push(o));
    assert_eq!(vec![1], outputs);
}

#[test]
fn test_nlt_8() {
    let mut computer = Interpreter::from_source("3,9,7,9,10,9,4,9,99,-1,8".to_string());
    let mut outputs = vec![];
    computer.run(|| Some(8), |o| outputs.push(o));
    assert_eq!(vec![0], outputs);
}

#[test]
fn test_eq_ii_8() {
    let mut computer = Interpreter::from_source("3,3,1108,-1,8,3,4,3,99".to_string());
    let mut outputs = vec![];
    computer.run(|| Some(8), |o| outputs.push(o));
    assert_eq!(vec![1], outputs);
}

#[test]
fn test_neq_ii_8() {
    let mut computer = Interpreter::from_source("3,3,1108,-1,8,3,4,3,99".to_string());
    let mut outputs = vec![];
    computer.run(|| Some(7), |o| outputs.push(o));
    assert_eq!(vec![0], outputs);
}

#[test]
fn test_jf_pp_z() {
    let mut computer =
        Interpreter::from_source("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9".to_string());
    let mut outputs = vec![];
    computer.run(|| Some(0), |o| outputs.push(o));
    assert_eq!(vec![0], outputs);
}

#[test]
fn test_jt_ii_z() {
    let mut computer = Interpreter::from_source("3,3,1105,-1,9,1101,0,0,12,4,12,99,1".to_string());
    let mut outputs = vec![];
    computer.run(|| Some(0), |o| outputs.push(o));
    assert_eq!(vec![0], outputs);
}

#[test]
fn test_jf_pp_nz() {
    let mut computer =
        Interpreter::from_source("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9".to_string());
    let mut outputs = vec![];
    computer.run(|| Some(42), |o| outputs.push(o));
    assert_eq!(vec![1], outputs);
}

#[test]
fn test_jt_ii_nz() {
    let mut computer = Interpreter::from_source("3,3,1105,-1,9,1101,0,0,12,4,12,99,1".to_string());
    let mut outputs = vec![];
    computer.run(|| Some(42), |o| outputs.push(o));
    assert_eq!(vec![1], outputs);
}
