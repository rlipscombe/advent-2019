mod interpreter;
mod amplifiers;

use interpreter::Interpreter;
use amplifiers::Amplifiers;

fn main() {
    let source = std::fs::read_to_string("test-data.txt").unwrap();
    let mut computer = Interpreter::from_source(source);
    computer.run(vec![5]);
}
