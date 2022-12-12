use std::vec;

#[derive(Debug)]
enum Operation {
    Addx(i32),
    Noop,
}

pub fn solve() {
    let mut iterations: Vec<Operation> = vec![];
    for line in get_input().lines() {
        let mut operation: Vec<Operation> = match line.split_at(4) {
            ("addx", n) => {
                vec![Operation::Addx(0), Operation::Addx(n.trim().parse::<i32>().unwrap())]
            }
            ("noop", _) => {
                vec![Operation::Noop]
            }
            _ => unreachable!(),
        };
        iterations.append(&mut operation);
    }

    let mut value = 1;
    let mut cycle = 20;
    let mut result = 0;
    for (i, iteration) in iterations.iter().enumerate() {
        if i + 1 == cycle {
            result += cycle as i32 * value;
            cycle += 40;
        }
        match iteration {
            Operation::Addx(n) => { value += n }
            Operation::Noop => {}
        }
    }
    println!("result: {:?}", result);
}

fn get_input() -> String {
    return String::from("noop
noop
noop
addx 5
noop
addx 1
addx 2
addx 5
addx 2
addx 1
noop
addx 5
noop
addx -1
noop
addx 5
noop
noop
addx 5
addx 1
noop
noop
addx 3
addx 2
noop
addx -38
noop
addx 3
addx 2
addx -5
addx 12
addx 2
addx 27
addx -40
addx 19
addx 2
addx 19
addx -18
addx 2
addx 5
addx 2
addx -23
addx 22
addx 4
addx -34
addx -1
addx 5
noop
addx 2
addx 1
addx 20
addx -17
noop
addx 25
addx -17
addx -2
noop
addx 3
addx 19
addx -12
addx 3
addx -2
addx 3
addx 1
noop
addx 5
noop
noop
addx -37
addx 3
addx 4
noop
addx 24
addx -6
addx -15
addx 2
noop
addx 6
addx -2
addx 6
addx -12
addx -2
addx 19
noop
noop
noop
addx 3
noop
addx 7
addx -2
addx -24
addx -11
addx 4
addx 3
addx -2
noop
addx 7
addx -2
addx 2
noop
addx 3
addx 7
noop
addx -2
addx 5
addx 2
addx 5
noop
noop
noop
addx 3
addx -35
addx 35
addx -21
addx -14
noop
addx 5
addx 2
addx 33
addx -7
addx -23
addx 5
addx 2
addx 1
noop
noop
addx 5
addx -1
noop
addx 3
addx -23
addx 30
addx 1
noop
addx 4
addx -17
addx 11
noop
noop
");
}
