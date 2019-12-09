mod interpreter;

use interpreter::Interpreter;

fn main() {
    let source = std::fs::read_to_string("input.txt").unwrap();
    let mut computer = Interpreter::from_source(source);
    let mut outputs = vec![];
    computer.run(|| { None }, |o| {outputs.push(o)});
    println!("{:?}", outputs);
}
