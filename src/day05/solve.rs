use std::collections::{BTreeMap, VecDeque};

pub fn solve() {
    let mut cargo: BTreeMap<i32, VecDeque<char>> = BTreeMap::new();
    let mut operations: Vec<i32> = vec![];
    let mut last_cargo_index = 0;
    get_input()
        .lines()
        .enumerate()
        .for_each(|(i, line)| {
            if last_cargo_index != 0 { return; }
            // 4 in 4 we get a crate starting at 1
            let mut index = 0;
            line.chars().skip(1).step_by(4).for_each(|c| {
                if c.is_alphabetic() {
                    if !cargo.contains_key(&(index + 1)) {
                        cargo.insert(index + 1, VecDeque::new());
                    }

                    cargo.get_mut(&(index + 1)).unwrap().push_back(c);
                } else if c.is_numeric() {
                    last_cargo_index = i + 2;
                    return;
                }
                index += 1;
            });
        });

    get_input().lines().skip(last_cargo_index).for_each(|line| {
        // we do the operations
        // care about token of index 1,3,5
        line.split_whitespace().skip(1).step_by(2).for_each(|token| {
            operations.push(token.parse::<i32>().unwrap());
        });

        let agg_op = operations.get(0).unwrap();
        let source_op = operations.get(1).unwrap();
        let dest_op = operations.get(2).unwrap();

        let mut source_move = vec![];

        for _ in 0..*agg_op {
            let source_crate = cargo.get_mut(source_op).unwrap().pop_front().unwrap();
            source_move.push(source_crate);
        }

        for n in (0..*agg_op).rev() {
            let moving_create = source_move.get(n as usize).unwrap().clone();
            cargo.get_mut(dest_op).unwrap().push_front(moving_create);
        }

        // clear them for the next iteration
        operations.clear();
    });

    let result = cargo.values().fold(String::new(), |mut acc, q| {
        let c = q.front().unwrap().clone();
        acc.push(c);
        return acc;
    });
    println!("result: {:?}", result);
}

fn get_input() -> String {
    return String::from("            [M] [S] [S]
        [M] [N] [L] [T] [Q]
[G]     [P] [C] [F] [G] [T]
[B]     [J] [D] [P] [V] [F] [F]
[D]     [D] [G] [C] [Z] [H] [B] [G]
[C] [G] [Q] [L] [N] [D] [M] [D] [Q]
[P] [V] [S] [S] [B] [B] [Z] [M] [C]
[R] [H] [N] [P] [J] [Q] [B] [C] [F]
 1   2   3   4   5   6   7   8   9

move 1 from 7 to 4
move 3 from 4 to 7
move 4 from 3 to 4
move 5 from 6 to 9
move 1 from 8 to 1
move 2 from 3 to 2
move 3 from 4 to 6
move 1 from 3 to 6
move 9 from 7 to 1
move 1 from 2 to 4
move 3 from 4 to 9
move 4 from 9 to 8
move 6 from 8 to 2
move 1 from 8 to 6
move 1 from 4 to 1
move 11 from 1 to 7
move 1 from 4 to 7
move 7 from 2 to 5
move 5 from 6 to 3
move 2 from 4 to 3
move 2 from 5 to 9
move 1 from 8 to 6
move 3 from 1 to 5
move 2 from 6 to 9
move 1 from 4 to 8
move 2 from 2 to 1
move 7 from 5 to 9
move 6 from 3 to 6
move 1 from 2 to 5
move 1 from 3 to 8
move 12 from 7 to 3
move 1 from 1 to 8
move 2 from 1 to 9
move 20 from 9 to 5
move 1 from 1 to 7
move 5 from 5 to 3
move 1 from 8 to 7
move 2 from 8 to 3
move 2 from 6 to 5
move 1 from 6 to 4
move 18 from 3 to 2
move 1 from 4 to 2
move 1 from 7 to 9
move 1 from 1 to 9
move 1 from 6 to 1
move 8 from 5 to 2
move 1 from 1 to 6
move 19 from 5 to 2
move 5 from 2 to 6
move 2 from 9 to 7
move 20 from 2 to 1
move 1 from 9 to 4
move 8 from 6 to 2
move 5 from 1 to 3
move 27 from 2 to 1
move 34 from 1 to 7
move 1 from 2 to 6
move 2 from 3 to 1
move 1 from 4 to 9
move 1 from 2 to 6
move 2 from 1 to 7
move 1 from 6 to 7
move 1 from 9 to 3
move 2 from 6 to 3
move 1 from 6 to 4
move 5 from 3 to 4
move 5 from 4 to 2
move 4 from 1 to 4
move 4 from 1 to 4
move 2 from 3 to 6
move 1 from 6 to 9
move 25 from 7 to 5
move 1 from 6 to 4
move 17 from 5 to 2
move 14 from 7 to 6
move 1 from 7 to 3
move 1 from 9 to 3
move 10 from 2 to 3
move 2 from 2 to 9
move 12 from 3 to 5
move 18 from 5 to 8
move 1 from 4 to 2
move 5 from 2 to 1
move 1 from 5 to 3
move 3 from 2 to 1
move 1 from 2 to 7
move 2 from 2 to 6
move 9 from 6 to 7
move 16 from 8 to 2
move 6 from 7 to 3
move 2 from 8 to 9
move 1 from 1 to 8
move 16 from 2 to 6
move 1 from 8 to 3
move 2 from 1 to 2
move 1 from 9 to 3
move 7 from 4 to 1
move 7 from 3 to 7
move 9 from 7 to 8
move 1 from 5 to 6
move 2 from 9 to 7
move 1 from 9 to 5
move 11 from 6 to 8
move 1 from 3 to 1
move 1 from 4 to 2
move 2 from 8 to 5
move 3 from 5 to 8
move 2 from 1 to 9
move 5 from 1 to 4
move 3 from 4 to 1
move 1 from 3 to 2
move 3 from 2 to 1
move 1 from 9 to 1
move 1 from 2 to 5
move 2 from 4 to 7
move 20 from 8 to 5
move 1 from 9 to 7
move 11 from 6 to 1
move 17 from 1 to 5
move 1 from 8 to 2
move 7 from 5 to 8
move 1 from 4 to 5
move 2 from 1 to 2
move 2 from 8 to 4
move 4 from 7 to 6
move 2 from 6 to 8
move 2 from 1 to 2
move 1 from 1 to 4
move 4 from 8 to 9
move 2 from 1 to 9
move 3 from 8 to 1
move 25 from 5 to 2
move 23 from 2 to 1
move 1 from 7 to 1
move 6 from 9 to 8
move 6 from 8 to 3
move 3 from 6 to 2
move 10 from 1 to 2
move 1 from 6 to 3
move 2 from 3 to 6
move 2 from 3 to 2
move 2 from 6 to 8
move 1 from 4 to 6
move 14 from 1 to 9
move 2 from 3 to 4
move 14 from 2 to 4
move 1 from 6 to 9
move 17 from 4 to 3
move 1 from 8 to 6
move 2 from 7 to 2
move 1 from 4 to 2
move 1 from 5 to 9
move 9 from 2 to 4
move 17 from 3 to 7
move 3 from 4 to 2
move 1 from 8 to 3
move 4 from 5 to 7
move 1 from 3 to 6
move 1 from 4 to 5
move 14 from 7 to 9
move 2 from 1 to 9
move 3 from 2 to 1
move 1 from 2 to 5
move 1 from 3 to 7
move 4 from 1 to 2
move 2 from 6 to 7
move 3 from 9 to 8
move 4 from 2 to 4
move 17 from 9 to 7
move 1 from 2 to 8
move 8 from 9 to 6
move 1 from 8 to 2
move 19 from 7 to 9
move 9 from 4 to 2
move 5 from 7 to 3
move 3 from 5 to 9
move 6 from 2 to 5
move 1 from 9 to 4
move 3 from 2 to 9
move 25 from 9 to 5
move 1 from 3 to 6
move 2 from 5 to 8
move 6 from 6 to 7
move 1 from 3 to 4
move 2 from 3 to 4
move 1 from 8 to 2
move 2 from 2 to 9
move 2 from 8 to 3
move 5 from 7 to 6
move 3 from 7 to 9
move 7 from 5 to 8
move 2 from 3 to 5
move 1 from 3 to 5
move 1 from 6 to 2
move 6 from 9 to 5
move 1 from 9 to 2
move 1 from 6 to 9
move 2 from 5 to 6
move 2 from 9 to 8
move 11 from 8 to 1
move 2 from 5 to 9
move 3 from 6 to 5
move 1 from 4 to 7
move 5 from 5 to 7
move 1 from 4 to 8
move 7 from 7 to 2
move 12 from 5 to 2
move 10 from 1 to 8
move 1 from 9 to 6
move 3 from 8 to 1
move 1 from 1 to 6
move 10 from 2 to 3
move 8 from 8 to 7
move 1 from 9 to 8
move 2 from 3 to 5
move 14 from 5 to 8
move 1 from 3 to 2
move 3 from 8 to 1
move 3 from 8 to 4
move 3 from 2 to 4
move 5 from 6 to 4
move 8 from 7 to 9
move 6 from 8 to 7
move 1 from 5 to 7
move 6 from 2 to 9
move 4 from 4 to 6
move 4 from 4 to 9
move 3 from 9 to 3
move 1 from 8 to 6
move 1 from 5 to 6
move 2 from 7 to 2
move 1 from 3 to 4
move 3 from 4 to 1
move 3 from 4 to 3
move 5 from 6 to 4
move 4 from 3 to 8
move 1 from 6 to 4
move 8 from 3 to 2
move 2 from 8 to 5
move 11 from 9 to 7
move 9 from 1 to 9
move 2 from 7 to 3
move 1 from 6 to 8
move 1 from 6 to 5
move 5 from 9 to 8
move 3 from 9 to 7
move 2 from 9 to 1
move 2 from 3 to 7
move 12 from 7 to 1
move 2 from 8 to 9
move 5 from 4 to 5
move 4 from 9 to 4
move 1 from 1 to 3
move 7 from 1 to 3
move 7 from 5 to 6
move 1 from 9 to 1
move 1 from 5 to 1
move 5 from 7 to 8
move 4 from 6 to 7
move 5 from 1 to 8
move 1 from 4 to 3
move 12 from 8 to 7
move 2 from 2 to 4
move 2 from 8 to 9
move 3 from 8 to 2
move 2 from 6 to 7
move 4 from 7 to 8
move 1 from 6 to 8
move 4 from 3 to 2
move 15 from 7 to 8
move 1 from 7 to 6
move 3 from 3 to 5
move 2 from 3 to 4
move 5 from 2 to 5
move 3 from 1 to 5
move 4 from 5 to 6
move 4 from 5 to 9
move 1 from 5 to 7
move 4 from 9 to 4
move 2 from 2 to 9
move 2 from 5 to 2
move 2 from 2 to 1
move 3 from 4 to 9
move 2 from 9 to 4
move 2 from 8 to 5
move 2 from 5 to 2
move 8 from 2 to 4
move 2 from 1 to 3
move 2 from 3 to 5
move 3 from 6 to 9
move 2 from 6 to 1
move 2 from 1 to 4
move 1 from 2 to 4
move 1 from 5 to 7
move 2 from 2 to 7
move 18 from 4 to 2
move 1 from 5 to 9
move 2 from 7 to 9
move 18 from 8 to 4
move 1 from 7 to 8
move 22 from 4 to 8
move 6 from 2 to 6
move 3 from 6 to 8
move 3 from 6 to 4
move 3 from 4 to 7
move 3 from 7 to 1
move 14 from 2 to 3
move 10 from 3 to 2
move 27 from 8 to 1
move 1 from 7 to 6
move 1 from 3 to 7
move 2 from 2 to 8
move 2 from 9 to 8
move 18 from 1 to 4
move 6 from 1 to 5
move 10 from 4 to 7
move 1 from 3 to 7
move 4 from 7 to 2
move 3 from 9 to 7
move 1 from 6 to 5
move 1 from 2 to 7
move 2 from 5 to 6
move 2 from 6 to 5
move 3 from 5 to 1
move 6 from 1 to 3
move 4 from 5 to 9
move 11 from 2 to 9
move 2 from 1 to 6
move 3 from 4 to 6
move 5 from 7 to 3
move 2 from 6 to 1
move 2 from 1 to 5
move 1 from 8 to 2
move 1 from 1 to 8
move 1 from 6 to 4
move 2 from 4 to 5
move 4 from 5 to 9
move 11 from 3 to 6
move 1 from 3 to 6
move 8 from 6 to 5
move 1 from 3 to 5
move 4 from 4 to 8
move 21 from 9 to 6
move 2 from 9 to 5
move 1 from 9 to 3
move 1 from 2 to 6
move 7 from 8 to 6
move 12 from 6 to 5
move 1 from 8 to 2
move 10 from 6 to 7
move 15 from 7 to 2
move 2 from 7 to 3
move 13 from 6 to 8
move 9 from 5 to 1
move 12 from 5 to 3
move 1 from 2 to 3
move 1 from 9 to 7
move 9 from 3 to 4
move 3 from 4 to 6
move 1 from 7 to 6
move 6 from 4 to 1
move 2 from 5 to 2
move 6 from 1 to 8
move 9 from 8 to 6
move 7 from 3 to 2
move 1 from 2 to 9
move 9 from 6 to 1
move 13 from 1 to 7
move 4 from 8 to 5
move 2 from 7 to 1
move 3 from 6 to 4
move 3 from 5 to 8
move 3 from 2 to 6
move 1 from 5 to 3
move 1 from 3 to 4
move 1 from 9 to 8
move 3 from 8 to 7
move 12 from 2 to 9
move 10 from 7 to 4
move 5 from 8 to 4
move 1 from 8 to 5
move 11 from 4 to 7
move 8 from 9 to 7
move 1 from 6 to 2
move 8 from 2 to 6
move 1 from 5 to 8
move 4 from 1 to 5
move 4 from 9 to 6
move 3 from 1 to 3
move 2 from 8 to 4
move 1 from 7 to 6
move 1 from 2 to 7
move 2 from 3 to 7
move 4 from 4 to 9
move 11 from 6 to 9
move 10 from 7 to 8
move 1 from 3 to 4
move 1 from 6 to 4
move 4 from 5 to 7
move 6 from 7 to 4
move 1 from 8 to 7
move 4 from 6 to 7
move 12 from 4 to 8
move 12 from 8 to 1
move 1 from 8 to 2
move 10 from 1 to 7
move 2 from 4 to 1
move 8 from 8 to 3
move 4 from 1 to 6
move 8 from 7 to 6
move 2 from 6 to 5
move 2 from 5 to 2
move 13 from 9 to 3
move 3 from 2 to 5
move 8 from 3 to 4
move 7 from 6 to 7
move 1 from 9 to 2
move 1 from 9 to 1
move 2 from 6 to 4
move 3 from 4 to 8
move 1 from 1 to 7
move 4 from 4 to 6
move 3 from 8 to 7
move 1 from 2 to 9
move 1 from 5 to 2
move 1 from 2 to 5
move 2 from 4 to 5
move 1 from 7 to 2
move 13 from 3 to 4
move 7 from 4 to 3
move 4 from 5 to 9
move 1 from 4 to 7
move 5 from 6 to 3
move 3 from 9 to 7
move 10 from 7 to 8
move 3 from 4 to 8
move 1 from 5 to 4
move 2 from 3 to 1
move 3 from 7 to 4
move 4 from 8 to 6
move 2 from 9 to 3
move 2 from 4 to 5
move 4 from 4 to 3
move 8 from 8 to 3
move 3 from 6 to 8
move 1 from 2 to 6
move 5 from 7 to 9
move 1 from 4 to 3
move 3 from 7 to 5
move 3 from 8 to 4
move 7 from 7 to 5
move 3 from 7 to 8
move 1 from 9 to 8
move 3 from 4 to 1
move 1 from 5 to 8
move 3 from 7 to 1
move 6 from 8 to 3
move 3 from 9 to 5
move 2 from 6 to 5
move 2 from 1 to 6
move 16 from 3 to 8
move 4 from 5 to 8
move 4 from 3 to 8
move 1 from 9 to 5
move 1 from 6 to 5
move 3 from 3 to 7
move 6 from 1 to 6
move 1 from 5 to 4
move 3 from 5 to 2
move 2 from 7 to 4
move 1 from 2 to 8
move 6 from 8 to 1
move 2 from 4 to 5
move 2 from 2 to 3
move 7 from 8 to 7
move 1 from 4 to 6
move 3 from 6 to 4
move 3 from 4 to 9
move 3 from 6 to 3
move 11 from 8 to 6
move 12 from 5 to 4
move 5 from 6 to 1
move 9 from 3 to 2
move 7 from 6 to 1
move 7 from 7 to 8
move 5 from 8 to 3
move 2 from 3 to 6
move 2 from 8 to 1
move 1 from 7 to 2
move 7 from 3 to 8
move 1 from 9 to 1
move 14 from 1 to 3
move 9 from 2 to 8
move 11 from 3 to 4
move 22 from 4 to 1
move 2 from 3 to 1
move 16 from 8 to 4
move 1 from 9 to 2
move 3 from 6 to 9
move 3 from 9 to 5
move 1 from 2 to 6
move 1 from 5 to 7
");
}
