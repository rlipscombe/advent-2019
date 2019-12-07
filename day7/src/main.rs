mod interpreter;
mod amplifiers;

use amplifiers::Amplifiers;

fn main() {
    let source = std::fs::read_to_string("test-data.txt").unwrap();
    let settings = vec![1,2,3,4,5];
    let amps = Amplifiers::new(source, settings);
    println!("{}", amps.run());
}
