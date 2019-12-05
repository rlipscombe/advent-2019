struct Interpreter {
    memory: Vec<i32>,
    ip: usize,
}

const OP_ADD_PP: i32 = 00_01;
const OP_ADD_IP: i32 = 01_01;
const OP_ADD_PI: i32 = 10_01;
const OP_ADD_II: i32 = 11_01;
const OP_SUB_PP: i32 = 00_02;
const OP_SUB_IP: i32 = 01_02;
const OP_SUB_PI: i32 = 10_02;
const OP_SUB_II: i32 = 11_02;
const OP_INPUT_P: i32 = 0_03;
const OP_OUTPUT_P: i32 = 0_04;
const OP_OUTPUT_I: i32 = 1_04;
const OP_HALT: i32 = 99;

impl Interpreter {
    fn new(bytes: Vec<i32>) -> Interpreter {
        Interpreter {
            memory: bytes,
            ip: 0,
        }
    }

    fn run(&mut self) {
        loop {
            match self.read_op(self.ip) {
                OP_ADD_PP => {
                    let lhs = self.read_via(self.ip + 1);
                    let rhs = self.read_via(self.ip + 2);
                    let trg_p = self.read(self.ip + 3) as usize;
                    let result = lhs + rhs;
                    println!(
                        "{} : @{} <- @{} + @{}; {} + {} = {}",
                        self.read_op(self.ip),
                        self.read(self.ip + 1),
                        self.read(self.ip + 2),
                        self.read(self.ip + 3),
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
                    self.write(trg_p, result);
                    self.ip += 4;
                }
                OP_ADD_IP => {
                    let lhs = self.read(self.ip + 1);
                    let rhs = self.read_via(self.ip + 2);
                    let trg_p = self.read(self.ip + 3) as usize;
                    let result = lhs + rhs;
                    self.write(trg_p, result);
                    self.ip += 4;
                }
                OP_ADD_PI => {
                    let lhs = self.read_via(self.ip + 1);
                    let rhs = self.read(self.ip + 2);
                    let trg_p = self.read(self.ip + 3) as usize;
                    let result = lhs + rhs;
                    self.write(trg_p, result);
                    self.ip += 4;
                }
                OP_SUB_PP => {
                    let lhs = self.read_via(self.ip + 1);
                    let rhs = self.read_via(self.ip + 2);
                    let trg_p = self.read(self.ip + 3) as usize;
                    let result = lhs - rhs;
                    self.write(trg_p, result);
                    self.ip += 4;
                }
                OP_SUB_IP => {
                    let lhs = self.read(self.ip + 1);
                    let rhs = self.read_via(self.ip + 2);
                    let trg_p = self.read(self.ip + 3) as usize;
                    let result = lhs - rhs;
                    self.write(trg_p, result);
                    self.ip += 4;
                }
                OP_SUB_PI => {
                    let lhs = self.read_via(self.ip + 1);
                    let rhs = self.read(self.ip + 2);
                    let trg_p = self.read(self.ip + 3) as usize;
                    let result = lhs - rhs;
                    self.write(trg_p, result);
                    self.ip += 4;
                }
                OP_SUB_II => {
                    let lhs = self.read(self.ip + 1);
                    let rhs = self.read(self.ip + 2);
                    let trg_p = self.read(self.ip + 3) as usize;
                    let result = lhs - rhs;
                    self.write(trg_p, result);
                    self.ip += 4;
                }
                OP_INPUT_P => {
                    let p = self.read(self.ip + 1) as usize;
                    let value = 1; // the only input
                    self.write(p, value);
                    self.ip += 2;
                }
                OP_OUTPUT_P => {
                    let value = self.read_via(self.ip + 1);
                    println!("{}", value);
                    self.ip += 2;
                }
                OP_OUTPUT_I => {
                    let value = self.read(self.ip + 1);
                    println!("{}", value);
                    self.ip += 2;
                }
                OP_HALT => return,
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

fn main() {
    let source = std::fs::read_to_string("test-data.txt").unwrap();
    let bytes = parse(source);
    let mut computer = Interpreter::new(bytes);
    computer.run();
}

// A given value could be code or data; leave this as a Vec.
fn parse(source: String) -> Vec<i32> {
    source
        .trim_end()
        .split(",")
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
}
