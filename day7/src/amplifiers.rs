use std::thread;
use std::sync::mpsc;
use std::time::Duration;

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
        let (s0, ra) = mpsc::channel();
        let (sa, rb) = mpsc::channel();
        let (sb, rc) = mpsc::channel();
        let (sc, rd) = mpsc::channel();
        let (sd, re) = mpsc::channel();
        let (se, r0) = mpsc::channel();
        spawn("A".to_string(), &self.source, ra, sa.clone());
        spawn("B".to_string(), &self.source, rb, sb.clone());
        spawn("C".to_string(), &self.source, rc, sc.clone());
        spawn("D".to_string(), &self.source, rd, sd.clone());
        spawn("E".to_string(), &self.source, re, se.clone());

        s0.send(self.settings[0]).unwrap();
        sa.send(self.settings[1]).unwrap();
        sb.send(self.settings[2]).unwrap();
        sc.send(self.settings[3]).unwrap();
        sd.send(self.settings[4]).unwrap();

        s0.send(0).unwrap();
        let v = r0.recv().unwrap();
        v
    }
}

fn spawn(name: String, source: &String, input: mpsc::Receiver<i32>, output: mpsc::Sender<i32>) {
    let mut i = Interpreter::from_source(source.clone());
    thread::Builder::new().name(name).spawn(move || {
        println!("spawning");
        let inp = || { input.recv().ok() };
        let outp = |x| { output.send(x).unwrap(); };
        i.run(inp, outp);
    }).unwrap();
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
    assert_eq!(54321, amplifiers.run());
}

#[test]
fn test_amplifiers_3() {
    let settings = vec![1, 0, 4, 3, 2];
    let program = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0";
    let amplifiers = Amplifiers::new(&program.to_string(), settings);
    assert_eq!(65210, amplifiers.run());
}
