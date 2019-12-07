pub struct Interpreter {
    memory: Vec<i32>,
    ip: usize,
}

const OP_ADD_PP: i32 = 00_01;
const OP_ADD_IP: i32 = 01_01;
const OP_ADD_PI: i32 = 10_01;
const OP_ADD_II: i32 = 11_01;
const OP_MUL_PP: i32 = 00_02;
const OP_MUL_IP: i32 = 01_02;
const OP_MUL_PI: i32 = 10_02;
const OP_MUL_II: i32 = 11_02;
const OP_INPUT_P: i32 = 0_03;
const OP_OUTPUT_P: i32 = 0_04;
const OP_OUTPUT_I: i32 = 1_04;
//const OP_JT_PP: i32 = 00_05;
const OP_JT_IP: i32 = 01_05;
const OP_JT_PI: i32 = 10_05;
const OP_JT_II: i32 = 11_05;
const OP_JF_PP: i32 = 00_06;
const OP_JF_IP: i32 = 01_06;
const OP_JF_PI: i32 = 10_06;
const OP_JF_II: i32 = 11_06;
const OP_LT_PP: i32 = 00_07;
const OP_LT_IP: i32 = 01_07;
const OP_LT_PI: i32 = 10_07;
const OP_LT_II: i32 = 11_07;
const OP_EQ_PP: i32 = 00_08;
const OP_EQ_IP: i32 = 01_08;
const OP_EQ_PI: i32 = 10_08;
const OP_EQ_II: i32 = 11_08;
const OP_HALT: i32 = 99;

impl Interpreter {
    pub fn from_source(source: String) -> Interpreter {
    let bytes = source
        .trim_end()
        .split(",")
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
        Interpreter::new(bytes)
    }

    pub fn new(bytes: Vec<i32>) -> Interpreter {
        Interpreter {
            memory: bytes,
            ip: 0,
        }
    }

    pub fn run(&mut self, inputs: Vec<i32>) -> Vec<i32> {
        let mut input_cursor = 0;
        let mut outputs: Vec<i32> = Vec::new();
        loop {
            match self.read_op(self.ip) {
                OP_ADD_PP => {
                    let lhs = self.read_via(self.ip + 1);
                    let rhs = self.read_via(self.ip + 2);
                    let trg_p = self.read(self.ip + 3) as usize;
                    let result = lhs + rhs;
                    println!(
                        "{}: [{}] <- [{}] + [{}]  ; {} + {} = {}",
                        self.ip,
                        self.read(self.ip + 3),
                        self.read(self.ip + 1),
                        self.read(self.ip + 2),
                        lhs,
                        rhs,
                        result
                    );
                    self.write(trg_p, result);
                    self.ip += 4;
                }
                OP_ADD_II => {
                    let lhs = self.read(self.ip + 1);
                    let rhs = self.read(self.ip + 2);
                    let trg_p = self.read(self.ip + 3) as usize;
                    let result = lhs + rhs;
                    println!(
                        "{}: [{}] <- {} + {}  ; {} + {} = {}",
                        self.ip,
                        self.read(self.ip + 3),
                        self.read(self.ip + 1),
                        self.read(self.ip + 2),
                        lhs,
                        rhs,
                        result
                    );
                    self.write(trg_p, result);
                    self.ip += 4;
                }
                OP_ADD_IP => {
                    let lhs = self.read(self.ip + 1);
                    let rhs = self.read_via(self.ip + 2);
                    let trg_p = self.read(self.ip + 3) as usize;
                    let result = lhs + rhs;
                    println!(
                        "{}: [{}] <- {} + [{}]  ; {} + {} = {}",
                        self.ip,
                        self.read(self.ip + 3),
                        self.read(self.ip + 1),
                        self.read(self.ip + 2),
                        lhs,
                        rhs,
                        result
                    );
                    self.write(trg_p, result);
                    self.ip += 4;
                }
                OP_ADD_PI => {
                    let lhs = self.read_via(self.ip + 1);
                    let rhs = self.read(self.ip + 2);
                    let trg_p = self.read(self.ip + 3) as usize;
                    let result = lhs + rhs;
                    println!(
                        "{}: [{}] <- [{}] + {}  ; {} + {} = {}",
                        self.ip,
                        self.read(self.ip + 3),
                        self.read(self.ip + 1),
                        self.read(self.ip + 2),
                        lhs,
                        rhs,
                        result
                    );
                    self.write(trg_p, result);
                    self.ip += 4;
                }
                OP_MUL_PP => {
                    let lhs = self.read_via(self.ip + 1);
                    let rhs = self.read_via(self.ip + 2);
                    let trg_p = self.read(self.ip + 3) as usize;
                    let result = lhs * rhs;
                    println!(
                        "{}: [{}] <- [{}] * [{}]  ; {} * {} = {}",
                        self.ip,
                        self.read(self.ip + 3),
                        self.read(self.ip + 1),
                        self.read(self.ip + 2),
                        lhs,
                        rhs,
                        result
                    );
                    self.write(trg_p, result);
                    self.ip += 4;
                }
                OP_MUL_IP => {
                    let lhs = self.read(self.ip + 1);
                    let rhs = self.read_via(self.ip + 2);
                    let trg_p = self.read(self.ip + 3) as usize;
                    let result = lhs * rhs;
                    println!(
                        "{}: [{}] <- {} * [{}]  ; {} * {} = {}",
                        self.ip,
                        self.read(self.ip + 3),
                        self.read(self.ip + 1),
                        self.read(self.ip + 2),
                        lhs,
                        rhs,
                        result
                    );
                    self.write(trg_p, result);
                    self.ip += 4;
                }
                OP_MUL_PI => {
                    let lhs = self.read_via(self.ip + 1);
                    let rhs = self.read(self.ip + 2);
                    let trg_p = self.read(self.ip + 3) as usize;
                    let result = lhs * rhs;
                    println!(
                        "{}: [{}] <- [{}] * {}  ; {} * {} = {}",
                        self.ip,
                        self.read(self.ip + 3),
                        self.read(self.ip + 1),
                        self.read(self.ip + 2),
                        lhs,
                        rhs,
                        result
                    );
                    self.write(trg_p, result);
                    self.ip += 4;
                }
                OP_MUL_II => {
                    let lhs = self.read(self.ip + 1);
                    let rhs = self.read(self.ip + 2);
                    let trg_p = self.read(self.ip + 3) as usize;
                    let result = lhs * rhs;
                    println!(
                        "{}: [{}] <- {} * {}  ; {} * {} = {}",
                        self.ip,
                        self.read(self.ip + 3),
                        self.read(self.ip + 1),
                        self.read(self.ip + 2),
                        lhs,
                        rhs,
                        result
                    );
                    self.write(trg_p, result);
                    self.ip += 4;
                }
                OP_INPUT_P => {
                    let p = self.read(self.ip + 1) as usize;
                    let value = inputs[input_cursor];
                    input_cursor += 1;
                    println!(
                        "{}: [{}] <- in  ; {}",
                        self.ip,
                        self.read(self.ip + 1),
                        value
                    );
                    self.write(p, value);
                    self.ip += 2;
                }
                OP_OUTPUT_P => {
                    let value = self.read_via(self.ip + 1);
                    println!("{}: out [{}] = {}", self.ip, self.read(self.ip + 1), value);
                    outputs.push(value);
                    self.ip += 2;
                }
                OP_OUTPUT_I => {
                    let value = self.read(self.ip + 1);
                    println!("{}: out {} = {}", self.ip, self.read(self.ip + 1), value);
                    outputs.push(value);
                    self.ip += 2;
                }
                OP_JT_IP => {
                    let val = self.read(self.ip + 1);
                    let dst = self.read_via(self.ip + 2) as usize;
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
                OP_JT_PI => {
                    let val = self.read_via(self.ip + 1);
                    let dst = self.read(self.ip + 2) as usize;
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
                OP_JT_II => {
                    let val = self.read(self.ip + 1);
                    let dst = self.read(self.ip + 2) as usize;
                    println!(
                        "{}: jt {} {}",
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
                OP_JF_PP => {
                    let val = self.read_via(self.ip + 1);
                    let dst = self.read_via(self.ip + 2) as usize;
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
                OP_JF_IP => {
                    let val = self.read(self.ip + 1);
                    let dst = self.read_via(self.ip + 2) as usize;
                    println!(
                        "{}: jf [{}] {}",
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
                OP_JF_PI => {
                    let val = self.read_via(self.ip + 1);
                    let dst = self.read(self.ip + 2) as usize;
                    println!(
                        "{}: jf [{}] {}",
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
                OP_JF_II => {
                    let val = self.read(self.ip + 1);
                    let dst = self.read(self.ip + 2) as usize;
                    println!(
                        "{}: jf {} {}",
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
                OP_EQ_PP => {
                    let lhs = self.read_via(self.ip + 1);
                    let rhs = self.read_via(self.ip + 2);
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
                    self.write(trg_p, result as i32);
                    self.ip += 4;
                }
                OP_EQ_IP => {
                    let lhs = self.read(self.ip + 1);
                    let rhs = self.read_via(self.ip + 2);
                    let trg_p = self.read(self.ip + 3) as usize;
                    let result = lhs == rhs;
                    println!(
                        "{}: [{}] <- {} eq [{}]  ; {} == {} = {}",
                        self.ip,
                        self.read(self.ip + 3),
                        self.read(self.ip + 1),
                        self.read(self.ip + 2),
                        lhs,
                        rhs,
                        result
                    );
                    self.write(trg_p, result as i32);
                    self.ip += 4;
                }
                OP_EQ_PI => {
                    let lhs = self.read_via(self.ip + 1);
                    let rhs = self.read(self.ip + 2);
                    let trg_p = self.read(self.ip + 3) as usize;
                    let result = lhs == rhs;
                    println!(
                        "{}: [{}] <- [{}] eq {}  ; {} == {} = {}",
                        self.ip,
                        self.read(self.ip + 3),
                        self.read(self.ip + 1),
                        self.read(self.ip + 2),
                        lhs,
                        rhs,
                        result
                    );
                    self.write(trg_p, result as i32);
                    self.ip += 4;
                }
                OP_EQ_II => {
                    let lhs = self.read(self.ip + 1);
                    let rhs = self.read(self.ip + 2);
                    let trg_p = self.read(self.ip + 3) as usize;
                    let result = lhs == rhs;
                    println!(
                        "{}: [{}] <- {} eq {}  ; {} == {} = {}",
                        self.ip,
                        self.read(self.ip + 3),
                        self.read(self.ip + 1),
                        self.read(self.ip + 2),
                        lhs,
                        rhs,
                        result
                    );
                    self.write(trg_p, result as i32);
                    self.ip += 4;
                }
                OP_LT_PP => {
                    let lhs = self.read_via(self.ip + 1);
                    let rhs = self.read_via(self.ip + 2);
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
                    self.write(trg_p, result as i32);
                    self.ip += 4;
                }
                OP_LT_IP => {
                    let lhs = self.read(self.ip + 1);
                    let rhs = self.read_via(self.ip + 2);
                    let trg_p = self.read(self.ip + 3) as usize;
                    let result = lhs < rhs;
                    println!(
                        "{}: [{}] <- {} lt [{}]  ; {} lt {} = {}",
                        self.ip,
                        self.read(self.ip + 3),
                        self.read(self.ip + 1),
                        self.read(self.ip + 2),
                        lhs,
                        rhs,
                        result
                    );
                    self.write(trg_p, result as i32);
                    self.ip += 4;
                }
                OP_LT_PI => {
                    let lhs = self.read_via(self.ip + 1);
                    let rhs = self.read(self.ip + 2);
                    let trg_p = self.read(self.ip + 3) as usize;
                    let result = lhs < rhs;
                    println!(
                        "{}: [{}] <- [{}] lt {}  ; {} lt {} = {}",
                        self.ip,
                        self.read(self.ip + 3),
                        self.read(self.ip + 1),
                        self.read(self.ip + 2),
                        lhs,
                        rhs,
                        result
                    );
                    self.write(trg_p, result as i32);
                    self.ip += 4;
                }
                OP_LT_II => {
                    let lhs = self.read(self.ip + 1);
                    let rhs = self.read(self.ip + 2);
                    let trg_p = self.read(self.ip + 3) as usize;
                    let result = lhs < rhs;
                    println!(
                        "{}: [{}] <- {} lt {}  ; {} lt {} = {}",
                        self.ip,
                        self.read(self.ip + 3),
                        self.read(self.ip + 1),
                        self.read(self.ip + 2),
                        lhs,
                        rhs,
                        result
                    );
                    self.write(trg_p, result as i32);
                    self.ip += 4;
                }
                OP_HALT => {
                    println!("{}: halt", self.ip);
                    return outputs;
                }
                op => {
                    panic!("Invalid opcode {}", op);
                }
            }
        }
    }

    fn read(&self, p: usize) -> i32 {
        self.memory[p]
    }

    fn read_op(&self, p: usize) -> i32 {
        self.memory[p]
    }

    fn read_via(&self, p: usize) -> i32 {
        self.read(self.read(p) as usize)
    }

    fn write(&mut self, p: usize, v: i32) {
        self.memory[p] = v;
    }
}

#[test]
fn test_add_pp() {
    let mut computer = Interpreter::from_source("1,7,8,0,4,0,99,-12,12".to_string());
    assert_eq!(vec![0], computer.run(vec![]));
}

#[test]
fn test_add_ip() {
    let mut computer = Interpreter::from_source("101,-12,8,0,4,0,99,0,12".to_string());
    assert_eq!(vec![0], computer.run(vec![]));
}

#[test]
fn test_add_pi() {
    let mut computer = Interpreter::from_source("1001,8,-12,0,4,0,99,0,12".to_string());
    assert_eq!(vec![0], computer.run(vec![]));
}

#[test]
fn test_add_ii() {
    let mut computer = Interpreter::from_source("1101,123,-123,0,4,0,99".to_string());
    assert_eq!(vec![0], computer.run(vec![]));
}

#[test]
fn test_eq_8() {
    let mut computer = Interpreter::from_source("3,9,8,9,10,9,4,9,99,-1,8".to_string());
    assert_eq!(vec![1], computer.run(vec![8]));
}

#[test]
fn test_neq_8() {
    let mut computer = Interpreter::from_source("3,9,8,9,10,9,4,9,99,-1,8".to_string());
    assert_eq!(vec![0], computer.run(vec![-8]));
}

#[test]
fn test_lt_8() {
    let mut computer = Interpreter::from_source("3,9,7,9,10,9,4,9,99,-1,8".to_string());
    assert_eq!(vec![1], computer.run(vec![7]));
}

#[test]
fn test_nlt_8() {
    let mut computer = Interpreter::from_source("3,9,7,9,10,9,4,9,99,-1,8".to_string());
    assert_eq!(vec![0], computer.run(vec![8]));
}

#[test]
fn test_eq_ii_8() {
    let mut computer = Interpreter::from_source("3,3,1108,-1,8,3,4,3,99".to_string());
    assert_eq!(vec![1], computer.run(vec![8]));
}

#[test]
fn test_neq_ii_8() {
    let mut computer = Interpreter::from_source("3,3,1108,-1,8,3,4,3,99".to_string());
    assert_eq!(vec![0], computer.run(vec![7]));
}

#[test]
fn test_jf_pp_z() {
    let mut computer = Interpreter::from_source("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9".to_string());
    assert_eq!(vec![0], computer.run(vec![0]));
}

#[test]
fn test_jt_ii_z() {
    let mut computer = Interpreter::from_source("3,3,1105,-1,9,1101,0,0,12,4,12,99,1".to_string());
    assert_eq!(vec![0], computer.run(vec![0]));
}

#[test]
fn test_jf_pp_nz() {
    let mut computer = Interpreter::from_source("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9".to_string());
    assert_eq!(vec![1], computer.run(vec![42]));
}

#[test]
fn test_jt_ii_nz() {
    let mut computer = Interpreter::from_source("3,3,1105,-1,9,1101,0,0,12,4,12,99,1".to_string());
    assert_eq!(vec![1], computer.run(vec![42]));
}
