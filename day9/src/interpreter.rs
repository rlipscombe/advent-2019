use log::*;

pub struct Interpreter {
    memory: Vec<i64>,
    ip: usize,
    bp: i64,
}

#[derive(Debug)]
enum Value {
    Pos(usize),
    Imm(i64),
    Rel(i64),
}

#[derive(Debug)]
enum Target {
    Pos(usize),
    Rel(i64),
}

#[derive(Debug)]
enum Dest {
    Pos(usize),
    Imm(i64),
    Rel(i64),
}

#[derive(Debug)]
enum Instr {
    Add(Value, Value, Target),
    Mul(Value, Value, Target),
    In(Target),
    Out(Value),
    Jt(Value, Dest),
    Jf(Value, Dest),
    Eq(Value, Value, Target),
    Lt(Value, Value, Target),
    AdjBp(Value),
    Halt,
}

const OP_ADD: i64 = 1;
const OP_MUL: i64 = 2;
const OP_IN: i64 = 3;
const OP_OUT: i64 = 4;
const OP_JT: i64 = 5;
const OP_JF: i64 = 6;
const OP_LT: i64 = 7;
const OP_EQ: i64 = 8;
const OP_ADJBP: i64 = 9;
const OP_HALT: i64 = 99;

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
        println!("init, size = {}", bytes.len());
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
            trace!("{}: {}: {:?}", self.ip, self.memory[self.ip], instr);
            match instr {
                Instr::Add(lhs, rhs, trg) => {
                    self.put(trg, self.apply(|x, y| x + y, lhs, rhs));
                    self.ip += 4;
                }
                Instr::Mul(lhs, rhs, trg) => {
                    self.put(trg, self.apply(|x, y| x * y, lhs, rhs));
                    self.ip += 4;
                }
                Instr::In(trg) => {
                    let value = input().unwrap();
                    self.put(trg, value);
                    self.ip += 2;
                }
                Instr::Out(val) => {
                    output(self.get(val));
                    self.ip += 2;
                }
                Instr::Jt(val, dst) => {
                    if self.get(val) != 0 {
                        self.jmp(dst);
                    } else {
                        self.ip += 3;
                    }
                }
                Instr::Jf(val, dst) => {
                    if self.get(val) == 0 {
                        self.jmp(dst);
                    } else {
                        self.ip += 3;
                    }
                }
                Instr::Eq(lhs, rhs, trg) => {
                    self.put(
                        trg,
                        self.apply(
                            |x, y| {
                                if x == y {
                                    1
                                } else {
                                    0
                                }
                            },
                            lhs,
                            rhs,
                        ),
                    );
                    self.ip += 4;
                }
                Instr::Lt(lhs, rhs, trg) => {
                    self.put(
                        trg,
                        self.apply(
                            |x, y| {
                                if x < y {
                                    1
                                } else {
                                    0
                                }
                            },
                            lhs,
                            rhs,
                        ),
                    );
                    self.ip += 4;
                }
                Instr::AdjBp(val) => {
                    trace!("; bp <- bp + {:?}", val);
                    self.bp += self.get(val);
                    trace!("; bp <- {}", self.bp);
                    self.ip += 2;
                }
                Instr::Halt => {
                    return;
                }
            }
        }
    }

    fn apply<F>(&self, f: F, lhs: Value, rhs: Value) -> i64
    where
        F: Fn(i64, i64) -> i64,
    {
        f(self.get(lhs), self.get(rhs))
    }

    fn get(&self, val: Value) -> i64 {
        match val {
            Value::Imm(v) => v,
            Value::Pos(p) => self.read(p),
            Value::Rel(p) => self.read((self.bp + p) as usize),
        }
    }

    fn put(&mut self, trg: Target, val: i64) {
        match trg {
            Target::Pos(p) => self.write(p, val),
            Target::Rel(p) => self.write((self.bp + p) as usize, val),
        }
    }

    fn jmp(&mut self, dst: Dest) {
        match dst {
            Dest::Pos(p) => self.ip = self.read(p) as usize,
            Dest::Imm(p) => self.ip = p as usize,
            Dest::Rel(p) => self.ip = self.read((self.bp + p) as usize) as usize,
        }

        trace!("; ip <- {}", self.ip);
    }

    fn read(&self, p: usize) -> i64 {
        const INVALID_MEMORY: i64 = 66;
        let val = if self.memory.len() < p {
            INVALID_MEMORY
        } else {
            self.memory[p]
        };
        trace!("; [{}] = {}", p, val);
        val
    }

    fn read_instr(&self, p: usize) -> Instr {
        if self.memory.len() < p {
            Instr::Halt
        } else {
            let instr = self.memory[p];
            let op = instr % 100;
            let lmode = (instr / 100) % 10;
            let rmode = (instr / 1000) % 10;
            let tmode = (instr / 10000) % 10;

            match op {
                OP_ADD => {
                    let lhs = to_val(self.memory[p + 1], lmode);
                    let rhs = to_val(self.memory[p + 2], rmode);
                    let trg = to_trg(self.memory[p + 3], tmode);
                    Instr::Add(lhs, rhs, trg)
                }
               OP_MUL => {
                    let lhs = to_val(self.memory[p + 1], lmode);
                    let rhs = to_val(self.memory[p + 2], rmode);
                    let trg = to_trg(self.memory[p + 3], tmode);
                    Instr::Mul(lhs, rhs, trg)
                }
                OP_IN => {
                    let trg = to_trg(self.memory[p + 1], lmode);
                    Instr::In(trg)
                }
                OP_OUT => {
                    let val = to_val(self.memory[p + 1], lmode);
                    Instr::Out(val)
                }
                OP_JT => {
                    let val = to_val(self.memory[p + 1], lmode);
                    let dst = to_dst(self.memory[p + 2], rmode);
                    Instr::Jt(val, dst)
                }
                OP_JF => {
                    let val = to_val(self.memory[p + 1], lmode);
                    let dst = to_dst(self.memory[p + 2], rmode);
                    Instr::Jf(val, dst)
                }
                OP_EQ => {
                    let lhs = to_val(self.memory[p + 1], lmode);
                    let rhs = to_val(self.memory[p + 2], rmode);
                    let trg = to_trg(self.memory[p + 3], tmode);
                    Instr::Eq(lhs, rhs, trg)
                }
                OP_LT => {
                    let lhs = to_val(self.memory[p + 1], lmode);
                    let rhs = to_val(self.memory[p + 2], rmode);
                    let trg = to_trg(self.memory[p + 3], tmode);
                    Instr::Lt(lhs, rhs, trg)
                }
                OP_ADJBP => {
                    let val = to_val(self.memory[p + 1], lmode);
                    Instr::AdjBp(val)
                }
                OP_HALT => Instr::Halt,
                _ => panic!("{}: Invalid instruction {}", p, instr),
            }
        }
    }

    fn write(&mut self, p: usize, v: i64) {
        const INVALID_MEMORY: i64 = 66;
        while p > self.memory.len() {
            self.memory.resize(self.memory.len() * 2, INVALID_MEMORY);
        }
        trace!("; [{}] <- {}", p, v);
        self.memory[p] = v;
    }
}

fn to_val(val: i64, mode: i64) -> Value {
    match mode {
        0 => Value::Pos(val as usize),
        1 => Value::Imm(val),
        2 => Value::Rel(val),
        _ => {
            panic!("Invalid mode {}", mode);
        }
    }
}

fn to_trg(val: i64, mode: i64) -> Target {
    match mode {
        0 => Target::Pos(val as usize),
        2 => Target::Rel(val),
        _ => {
            panic!("Invalid mode {}", mode);
        }
    }
}

fn to_dst(val: i64, mode: i64) -> Dest {
    match mode {
        0 => Dest::Pos(val as usize),
        1 => Dest::Imm(val),
        2 => Dest::Rel(val),
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

#[test]
fn quine() {
    let source = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
    let mut computer = Interpreter::from_source(source.to_string());
    let mut outputs = vec![];
    computer.run(|| None, |o| outputs.push(o));
    assert_eq!(vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99], outputs);
}

#[test]
fn test_large_mul() {
    let source = "1102,34915192,34915192,7,4,7,99,0";
    let mut computer = Interpreter::from_source(source.to_string());
    let mut outputs = vec![];
    computer.run(|| None, |o| outputs.push(o));
    assert_eq!(vec![1219070632396864], outputs);
}

#[test]
fn test_large_output() {
    let source = "104,1125899906842624,99";
    let mut computer = Interpreter::from_source(source.to_string());
    let mut outputs = vec![];
    computer.run(|| None, |o| outputs.push(o));
    assert_eq!(vec![1125899906842624], outputs);
}