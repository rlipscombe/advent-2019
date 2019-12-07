mod amplifiers;
mod interpreter;

use itertools::Itertools;
use amplifiers::Amplifiers;

fn main() {
    let source = std::fs::read_to_string("test-data.txt").unwrap();

    let permutations = (0..5).permutations(5);

    let mut max = 0;
    for settings in permutations {
        let amps = Amplifiers::new(&source, settings);
        let v = amps.run();
        if v > max {
            max = v;
        }
    }
    println!("{}", max);
}
