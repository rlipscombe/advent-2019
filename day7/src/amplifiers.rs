use crate::interpreter::Interpreter;

pub struct Amplifiers {
    source: String,
    settings: Vec<i32>,
}

impl Amplifiers {
    pub fn new(source: &String, settings: Vec<i32>) -> Amplifiers {
        Amplifiers { source: source.clone(), settings }
    }

    pub fn run(self) -> i32 {
        let mut v = 0;
        for i in 0..5 {
            let p = self.settings[i];
            let mut computer = Interpreter::from_source(self.source.clone());
            let inputs = vec![p, v];
            let outputs = computer.run(inputs);
            v = outputs[0]
        }

        v
    }
}

#[test]
fn test_amplifiers_1() {
    let settings = vec![4, 3, 2, 1, 0];
    let program = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";
    let amplifiers = Amplifiers::new(&program.to_string(), settings);
    assert_eq!(43210, amplifiers.run());
}

#[test]
fn test_amplifiers_2() {
    let settings = vec![0, 1, 2, 3, 4];
    let program = "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0";
    let amplifiers = Amplifiers::new(&program.to_string(), settings);
    assert_eq!(54320, amplifiers.run());
}

#[test]
fn test_amplifiers_3() {
    let settings = vec![1, 0, 4, 3, 2];
    let program = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0";
    let amplifiers = Amplifiers::new(&program.to_string(), settings);
    assert_eq!(65210, amplifiers.run());
}
