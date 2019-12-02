use std::fs::read_to_string;

fn main() {
    let data = read_to_string("test-data.txt").unwrap();
    let mut ints: Vec<i32> = data
        .trim_end()
        .split(",")
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    run(&mut ints);
    println!("{:?}", ints);
}

fn run(ints: &mut Vec<i32>) {
    let mut ip = 0;
    loop {
        let op = ints[ip];
        match op {
            1 => {
                let lhs_p = ints[ip + 1] as usize;
                let rhs_p = ints[ip + 2] as usize;
                let trg_p = ints[ip + 3] as usize;
                let lhs = ints[lhs_p];
                let rhs = ints[rhs_p];
                let result = lhs + rhs;
                println!(
                    "[{}] <- [{}] + [{}] ; {} + {} = {}",
                    trg_p, lhs_p, rhs_p, lhs, rhs, result
                );
                ints[trg_p] = result;
                ip += 4;
            }
            2 => {
                let lhs_p = ints[ip + 1] as usize;
                let rhs_p = ints[ip + 2] as usize;
                let trg_p = ints[ip + 3] as usize;
                let lhs = ints[lhs_p];
                let rhs = ints[rhs_p];
                let result = lhs * rhs;
                println!(
                    "[{}] <- [{}] * [{}] ; {} * {} = {}",
                    trg_p, lhs_p, rhs_p, lhs, rhs, result
                );
                ints[trg_p] = result;
                ip += 4;
            }
            _ => {
                println!("{:?}", ints);
                return;
            }
        }
    }
}

#[test]
fn test_1() {
    let mut ints = vec![1, 0, 0, 0, 99];
    run(&mut ints);
    assert_eq!(vec![2, 0, 0, 0, 99], ints);
}

#[test]
fn test_2() {
    let mut ints = vec![2, 3, 0, 3, 99];
    run(&mut ints);
    assert_eq!(vec![2, 3, 0, 6, 99], ints);
}

#[test]
fn test_3() {
    let mut ints = vec![2, 4, 4, 5, 99, 0];
    run(&mut ints);
    assert_eq!(vec![2, 4, 4, 5, 99, 9801], ints);
}

#[test]
fn test_4() {
    let mut ints = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
    run(&mut ints);
    assert_eq!(vec![30, 1, 1, 4, 2, 5, 6, 0, 99], ints);
}
