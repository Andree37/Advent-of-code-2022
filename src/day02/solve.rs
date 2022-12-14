use anyhow::Result;

pub enum Decision {
    Play(u16), // 1,2,3 -> rock, paper, scissors
}


impl TryFrom<&str> for Decision {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self> {
        return if value == "A" || value == "X" {
            Ok(Decision::Play(1))
        } else if value == "B" || value == "Y" {
            Ok(Decision::Play(2))
        } else {
            Ok(Decision::Play(3))
        };
    }
}

fn get_result(value: Vec<Decision>) -> u16 {
    // we always expect the vec to have two elements
    let opponent = value.get(0).unwrap();
    let me = value.get(1).unwrap();
    return match opponent {
        Decision::Play(opp_play) => {
            match me {
                Decision::Play(my_play) => {
                    if *my_play == 1 {
                        // lose
                        let t = (opp_play + 2) % 3;
                        if t != 0 {
                            t
                        } else {
                            3
                        }
                    } else if *my_play == 2 {
                        // draw
                        opp_play + 3
                    } else {
                        // win
                        let t = opp_play + 1;
                        if t == 4 {
                            7
                        } else {
                            t + 6
                        }
                    }
                }
            }
        }
    };
}


pub fn solve1() {
    let result = get_input()
        .lines()
        .fold(0, |acc, elem| {
            let play = elem.split_whitespace();
            let decisions: Vec<Decision> = play.flat_map(|p| p.try_into()).collect();
            let result = get_result(decisions);
            println!("play:{}, result:{}", elem, result);
            return acc + result;
        });

    println!("day02 result: {}", result);
}

pub fn get_input() -> String {
    return String::from("A Y
A Y
B X
A Y
C Y
B Y
B Z
B Y
A X
B X
C Z
B Y
B Y
B Y
B Y
C X
A Z
B X
A Y
A Y
B Y
A X
C X
B Y
A Y
B Y
C X
A Z
B Y
A X
A Y
B Y
A Y
A X
B Y
B Y
B Y
B X
A X
C Y
A X
C Z
B Z
A Z
A Z
C X
A X
A X
B Y
B Y
C Y
B Y
B X
B Y
B Y
B X
C X
A X
A X
A Z
C X
A Z
A X
B Y
B Y
A Y
A Z
B Y
B X
B Y
B Y
B X
C Z
A X
B X
B X
A Y
B X
C Z
B Y
A X
C Z
A Y
B Y
B Y
A X
A Z
B Y
B Y
A Z
A Z
B Y
A X
A Z
C Y
A X
B Y
C X
A X
B Y
A X
A Z
C X
A Y
A X
B X
A X
B Z
B X
B Y
A X
A Y
B Y
B Z
C X
B X
A X
C Z
C Y
A Y
B Y
A X
B Y
C Y
B Y
C Y
B Y
C Z
C X
A Y
B Y
B X
A X
A X
A Y
A Y
B Y
A Z
B Y
A X
C X
B Y
B X
C X
A Y
B Z
B Y
B X
B Y
B Y
C Z
B X
B Z
A X
B Y
A X
B Y
C Z
A X
A Y
B Z
A X
B X
A Z
B Y
A Y
A X
B Y
C Z
B X
B Y
C X
B X
A X
C X
B X
B Y
B Z
B Y
B Y
A X
B Y
A X
A X
B X
A X
A X
B Y
B Y
B Y
C X
B Y
A Z
A X
B Y
A Y
A Z
B Y
A Z
C X
B Y
C X
B Y
B Y
A X
B Y
A X
B Y
B X
B Y
B Y
B Z
B Y
A Y
A X
A X
A X
C Z
A Y
A X
C Y
A X
B Z
B Y
B Y
B Y
A Z
B Y
B X
A Z
B Y
A Y
B Y
A Z
B Y
A X
B Z
A Z
C X
B Y
B Y
B Y
B X
A X
B X
A Y
B X
B Y
B Y
A X
B Y
B Z
B Y
B Y
B X
B X
C Z
A Y
B Y
B Y
B Y
B X
A X
B X
A Y
A Z
A Z
A Z
A X
A Y
B Y
C Y
A Z
A Z
A Z
C X
A X
C X
A Z
B X
A Y
B Y
A Z
B Y
B X
C Y
A Z
A X
C X
C X
C X
B Y
A X
A Y
C X
B Y
C Z
B Y
B Y
A Y
B Y
A Y
B Y
B Y
B X
A X
A X
A Z
A Y
B X
B Y
A Z
C X
A Y
A X
B Y
B Y
B X
A Z
B Y
A X
A X
B Y
A X
C X
A Z
A Z
B Y
A Z
A Y
A Y
A Y
B Y
A Y
A Z
A Y
A X
A Z
B Y
B Y
C X
B X
C Z
B X
A Z
C Y
B Y
A X
B Y
A Y
A X
B Y
B X
C Z
A Z
A Z
B Y
A Z
B Y
B Y
A X
A X
A X
B Z
A Y
B Z
A Y
C Y
A X
B X
A Y
B X
B Y
A X
A Y
A X
C X
C Z
B Y
A Y
B Y
B Z
A X
B Y
C Y
A Y
A Z
B Y
A X
A Z
A Y
B Y
B Y
A Z
C Z
B X
A Y
C Y
A X
A X
A X
B Y
C Z
A X
A X
B X
A X
A X
A Y
A X
C X
A X
A Y
B Z
A X
C X
A X
B X
A Y
A X
A X
A X
C Z
A X
A X
B X
B Y
A Y
A X
A Z
A Y
A Y
A Y
A Z
B X
A X
A X
A X
A Z
B Y
A X
A Y
B X
C X
B Y
A Z
A Z
A Z
B Z
B Y
A Z
A Z
A X
A Y
A X
A X
A X
A Y
A Z
B Y
A Z
B X
B X
A Y
A X
B X
B X
C Z
B X
A X
A Z
C X
A X
A X
B Z
A X
B X
A X
A X
A X
B Z
A Z
A Y
A Z
B Y
A X
A Y
A Y
B X
C Z
A X
A X
A X
B Y
B Y
A X
A X
B Y
A X
B X
A Y
C X
B X
B Y
C Z
B Y
A X
C Z
B Z
B Y
A Z
B X
B Y
B X
B Y
B Y
B Y
B X
B Y
A X
B Z
B Y
C X
B Y
C X
A Z
B X
A Z
A X
A Z
A Y
A X
A Z
B Y
B X
B X
B Y
B X
B X
B Z
B Z
B Y
A X
C X
B Y
B X
B X
C X
B X
B X
B Y
A Z
A Z
B Y
C Y
A Z
B Y
B Z
A Z
C Y
C X
A X
B Y
A Y
B X
B Y
B Z
A Y
A Z
A X
A X
C X
B Z
A Y
A Z
B Y
A Y
A Z
B Y
A Z
C Z
B Y
A X
A Z
B Y
B Y
A Y
A X
C Z
B Y
B X
A X
C Z
B Y
B Y
B Z
B Y
A Y
B Y
A Z
C Z
B Y
B X
B Z
B X
A X
A Y
A Z
A Z
A X
A X
B Y
B X
A Z
A X
B Y
B Y
B X
A Y
B Y
C Y
B Y
A X
B Y
B X
B Z
A X
B Y
B Y
A X
A X
A X
C X
B X
A Z
B X
C X
B Y
A Y
B Y
C X
B Y
B Y
B X
A X
A X
A X
A X
B Y
B Y
B Z
A Z
A Z
A Z
A Y
B X
A Y
B Y
C Z
C X
B Y
B Y
B Y
B Y
A X
A X
A Z
C Y
B X
A Y
B Y
B Y
C Z
B X
A X
A X
A Z
C X
B Y
B X
C Z
B X
A Y
B X
B X
B Z
C X
A Z
B Y
B X
B Y
B Y
A X
B Y
A X
A X
A Y
A Z
B Z
B Y
A Y
A Y
C Z
A Y
A X
B Y
B Y
C Z
B Y
A X
B X
A X
B Y
A X
A Z
B Z
B Y
A X
C X
A X
A X
B Y
A Y
A X
A Z
B Y
C Z
C X
A X
B X
B X
B Y
B X
B Y
B Y
A Z
A Y
A Z
A X
B Y
B X
B Y
C X
B Z
B X
C X
B Y
A X
A Y
A Y
B X
A X
B Y
B X
A X
B X
B Y
C Y
A Z
C Z
B X
B X
B Y
A Z
C Y
B X
A Z
B Y
A Y
A Z
B Y
A X
A Y
C Z
B Y
A X
B Y
B X
A Y
B Y
C X
A Y
B Y
B Y
A X
C Z
B Y
B X
A X
B X
B Y
A X
B Y
B Y
B Z
A X
C Z
A Y
B X
A X
C X
B Z
B Y
A Z
B X
A Z
B Y
C Z
A X
B Y
B X
B Y
A X
B Y
A X
A Y
A Y
A X
B Y
A X
B Y
A X
A Y
A Y
C X
B X
B Y
A Z
B X
A X
B Y
A Y
B X
B Y
A Z
B X
B X
A Y
B Y
A Z
B X
B Y
A Y
A X
A X
B X
C Z
A Y
B Y
A Z
A Z
A Z
B Y
A Y
A X
A Z
B X
B Y
C Y
A X
A X
C X
A Z
B X
B Y
B X
B Y
B Y
B Y
A X
B Z
B Z
B Z
B Y
A X
A X
B Y
A Z
B Y
C Y
B X
B X
B X
B Y
B Y
A Y
A X
B X
A X
A Z
A Y
C Y
B Y
B Z
B Z
A Y
B Y
A Z
B X
B Y
B Y
B X
B X
B Y
A X
C Z
B Y
A X
A X
A Y
B X
B Y
A Z
A Y
B Y
A Y
A X
A Z
A Y
A X
A X
A Y
B X
A X
B Y
B Y
B X
B Y
B Y
B Y
A Y
B X
A X
C Y
A Y
B Y
C X
C X
B Y
B Z
A Z
A Y
B Y
B Y
A Y
B Z
A Z
A X
B Y
A X
C Z
B Y
B Y
A Z
A Y
B Y
A Z
B Y
A Z
A Y
A Z
C Y
A Z
A X
A X
A Z
A X
B Y
A Z
B X
A Y
B Y
A Z
B X
B Z
B X
A Y
A Y
B Y
A Z
A X
B X
A Z
A X
B Y
B X
C X
A Z
C X
C X
B Z
A X
B Y
A Y
A Y
A Z
B Y
C X
A Z
A X
A Y
A Z
A X
A Y
A Y
A X
C X
B Y
B Y
B Y
A Z
B Y
A Z
B Y
C X
B Y
A X
B X
B Y
C X
A Z
B Y
B Y
B Y
A X
C Z
A Z
B Z
A X
A X
A X
A X
B Z
A Y
B X
B Z
A X
B X
A Y
A Y
C Z
B Y
B Y
A X
C Y
B X
B Y
A Y
C Z
A X
B X
A Z
C Z
C Z
B X
A Z
A X
B Z
A Z
B Y
B X
B Y
A Z
A Y
B Y
B Z
A X
B Z
A Z
A Y
C Y
A X
B Y
C Y
A Z
A Z
B X
B X
C X
A Y
B X
B X
B Y
A Y
C Z
A Z
A Y
B X
B Y
B Y
A X
C X
B X
A X
A Z
B Y
B Y
A X
A Y
B Y
C Z
A X
A X
B Y
A Y
A X
C Z
B X
C Y
A X
A X
A Y
B X
B Y
A X
B Y
A Y
B Y
B Y
B Y
B X
C X
A Z
A Z
A X
C Y
A Y
C X
A Z
A X
A X
B Y
A X
A X
B X
B Y
A Y
B Y
A X
B X
B Y
B Y
B X
C X
A Z
B Y
A Y
A Z
B Y
B Y
B Z
B Z
A X
B Y
A Y
A Z
A Y
A X
A Z
B X
A X
B X
A Y
C X
C X
A X
B X
A X
C X
A X
A Z
A Y
B Y
C X
A Y
A Y
B Y
B X
B Y
B Y
A Z
A Y
A Z
C X
B Y
B Y
A X
A X
B Y
A X
B Z
B X
B Y
A X
A Z
B Y
A Y
B Y
A X
C Z
A Z
A Y
B Y
B Y
B Y
B X
A Z
A Y
A X
B X
A Y
B Y
A Y
B Y
A Z
A Z
B Y
B X
B Y
A Y
C Y
B Z
B Y
A Y
C X
A Y
B Y
B Y
B Y
B Y
A X
B X
A Z
B Y
A Y
B Z
C X
B Y
A X
B X
B Y
B Y
A Y
A X
A Y
C X
B Y
A Y
A X
B Y
C X
A Y
B X
C Z
B Y
B Y
C Z
A X
A X
A X
A X
B Y
A Z
A Z
B X
A Z
A Z
A X
C Y
A Z
B Y
B Y
A Z
A Z
C Z
B Y
B X
C Z
C Z
B Y
B X
A Z
B X
C Z
C X
B Z
A Y
C Z
B Y
B Y
A Z
B Y
C X
C Y
B Y
C X
A X
A X
B Z
A Y
B Z
B Y
A X
B Z
A X
C X
B Y
A Y
A X
A X
B X
A X
A Y
A Y
B Y
B Z
B Y
A X
A Z
B Y
A Y
B Y
C Y
A X
B X
A X
B Y
A X
A X
B X
B Y
A Z
A Y
A X
C X
B X
C X
B Y
A X
B Y
A Z
A Z
C X
A Z
B Y
A Y
B Y
C X
B Y
B Z
A X
B Y
B X
A Z
B Z
B Y
A X
A Y
A Z
A Y
B Z
A X
A Y
A Z
A Y
C X
B Y
C Z
B Y
B Z
B X
B Y
A Z
C X
C Z
A Y
A Z
B Y
B X
B Y
B X
B X
A Y
A X
B X
B Y
A X
A X
B X
B Y
C X
A Y
C Y
A Z
C X
A Y
A Z
A Y
A X
A Z
B Y
C X
B Y
A Z
A Y
B Y
B Y
B X
B Y
C Z
A X
B X
A X
A X
A X
B Y
A X
B Z
B X
B Y
B Y
A X
B Z
A X
A Y
B Y
A X
B X
B Y
B X
A Z
A X
B Y
C X
B Y
A Z
B X
B Y
A Y
B X
A X
A X
A Y
A Y
B X
B Y
B Y
A X
A Y
B Y
A X
A X
A X
B X
B Y
A Y
A X
B Y
A X
B X
B Y
B Y
A Z
B X
B Y
B Z
A X
B X
A Z
A X
A Y
A X
B Y
B Y
A X
B Z
A X
B Y
B Z
A Y
A Y
A Z
A Y
B Y
A Y
B Y
A X
C Z
A Y
B X
B X
A Y
A X
B Y
C X
B X
B Z
A Y
A Y
C X
A X
B X
A X
A X
A Y
C X
A X
B Z
C Z
B X
B Y
A Z
C X
A X
B Y
B Y
A Y
B Z
B Y
B Y
A X
A X
C Z
B Y
B Y
B Y
A Z
A Z
B X
A X
B Y
A X
B Z
A X
B Y
B X
B Y
B Y
B Y
B Y
B Z
B Y
A Z
A X
C X
B Y
B Y
A X
B Y
B Y
A Z
A Z
A Z
A Z
A X
A Z
B X
B Y
B X
C Y
A Y
C X
B X
C X
A Z
B Y
B Y
A Y
B X
B Y
A Y
A X
A X
B Y
B X
A Y
B Y
B Y
C Z
C X
B X
B Y
A Y
A Z
B Y
A X
C Z
B Y
A Y
B Y
B Z
A X
A Z
A Y
A X
A Z
B Z
A Z
B Y
A Y
A X
A Z
B Y
B Y
B Y
B Y
B Y
A Z
B Y
C X
B Y
A Y
B X
B X
B Y
B Y
C X
C X
B Y
A Y
B Y
B Y
B X
A Z
B Y
B Y
B Y
A X
B Y
A Z
B Y
A X
B Y
B Y
A Z
B Y
A Z
C Z
B X
B Y
A Y
C Z
B Y
B X
B Y
A Z
A Z
B Y
C X
B Y
A Y
B Z
B Y
B Y
B X
A Z
A X
B Y
A X
A X
B X
B Y
A Z
B Y
C Z
A Z
A X
C Z
B Y
B X
C X
A X
A Z
A X
B Y
B Y
B X
A Z
B Y
B Y
C Z
A X
B Y
B Y
B X
B Y
B Y
C X
B Z
B Y
A X
B X
B X
B Y
A X
A Z
B Y
A X
A Z
A X
B Y
C Z
C X
C Z
A Z
B X
B Y
A X
B Z
A Y
A Z
A Z
A Z
A X
C Y
A X
B X
A Y
C Z
A X
B Y
B Y
A X
A Z
B Y
A Z
A X
A Z
B Y
A Y
B Y
A Y
C Y
B X
A Y
B Y
A X
B X
B Y
A Y
B Y
C Y
C X
B Y
B Y
B Z
B X
A Y
B Y
B Y
A X
C Y
C X
A X
B Y
B Y
B X
A X
B X
B Y
B Y
C X
B Y
B X
B X
A X
A X
B Y
B Y
B Y
B Y
B Y
C Z
A Y
B Y
C Z
B Y
C X
B Z
B Y
A X
A Z
B Y
A Y
A X
B Y
B Z
A X
C Y
B Y
B Y
A Z
A Y
B X
B Y
B Y
B Y
A Y
C Y
B X
B Y
B Y
B Y
B Y
B Y
A Y
B Y
A Z
B X
A Y
B X
B Y
B Y
A Z
B Z
B Y
C Y
A X
A Z
A X
B Y
C X
C Z
C Z
B Y
B Y
B Y
A Y
B X
B Y
C X
B Y
A Y
A Z
A X
A X
A Z
B Y
A Z
A X
B X
B Y
B Y
B Y
A Y
B Y
A Y
A Z
C Y
B X
C X
C X
A Y
B Y
A X
A X
A Y
A X
B Y
B Y
C X
B Y
C X
C X
C X
B Y
B X
B Z
A Y
C X
A Y
B Y
A Z
A Z
C Z
C X
A X
A X
A Z
B Y
C Z
B X
B Y
B Y
B Y
A Z
B Y
B X
B Y
B Y
A Z
A Z
B X
A X
B Z
A X
B Y
A Z
B Y
A Y
A X
B Y
B Y
C X
A X
B Y
C Y
B Y
B Y
A Y
A X
B X
B X
B Z
C X
B Y
B Y
C Y
B Y
A Y
B X
B Y
B X
A X
C X
C X
A X
A Y
A Y
B Y
A X
A X
B Y
A X
B Z
C X
B Y
A X
B Y
B Y
A Y
B Y
B Y
A X
B Z
B Y
B Y
A Z
B Z
B Z
B X
B Y
A Y
A Y
A X
B Y
A Y
B Y
A Z
B Y
A X
C Z
C Z
C X
B Y
B Z
B Z
C Z
B Z
B Y
C X
A Y
B Y
B Y
A X
C Z
A Y
A X
A Y
C X
A X
B X
C X
A Z
B Y
A X
A X
B Y
B Y
A X
B Y
A Z
A X
C X
B Y
C Z
B X
B Y
C Z
A X
B Z
A Y
A Y
B Y
C X
C Z
A X
A X
A Y
A Y
A Y
B Y
A X
A X
B Z
C Y
A Y
B Y
B Y
B Y
A X
A X
C Z
B Y
B Y
A X
B Y
B Y
A Y
A X
A Z
A Z
A Z
B X
A X
B Y
C Y
B Y
C Y
B X
A Y
C Z
A Y
B Y
A X
C X
A X
B Y
C X
A X
A X
B X
A Z
B X
C Z
A X
B X
A Y
C Y
C X
A X
A Y
A X
A X
B Y
A Z
A X
A Z
A Y
B Y
A Y
A X
B Y
A Y
A X
A X
B X
B Z
B Y
B X
C X
A Z
C X
B Y
A Y
B X
A Z
B Y
A Y
B Y
C X
B Y
B Y
C Z
B Z
B Y
A Z
A Y
B X
A X
A Z
A X
A Y
B X
A Z
A Y
A X
A X
B X
B Y
A X
C X
C Z
B Y
C X
B Y
A X
A X
B X
A X
A Z
B X
B Y
C X
B Y
B Y
A Z
A Y
A X
B X
A Z
A Y
C Z
A X
A X
A Z
C X
B Y
A Z
A Y
B Y
A Y
B X
B Y
C Y
B Y
C X
B Y
B Y
B Y
B X
C X
A X
B Y
B X
B X
B Y
A Z
B Y
A X
A X
A X
A X
C Y
A Z
A Y
C X
B X
A X
B Y
C X
C Z
B Y
A Z
B Z
B Y
B Y
A Y
B X
B Y
C X
B Y
A Y
B Z
A Z
A X
A X
A Y
C X
B Y
A Y
B X
B X
B Y
B Y
A Z
A Y
A Z
B Y
A Y
A X
B Y
C X
A X
B Y
A Y
A X
B Y
B Y
C Y
A X
A Z
A Y
B Y
B X
B X
B Y
A X
B Y
B X
A Y
B X
B Y
B Y
B Y
A Z
B Y
B Z
C X
A Z
B Y
B Y
C Y
A Y
B X
B Z
B X
A Z
B Y
B X
A Y
B X
B Z
A Z
C X
B X
B X
B Z
B Y
B Y
A Y
A Y
A Y
A Y
B Y
B Z
C X
A X
B Y
B X
A Y
C Z
B Y
A X
A Z
C X
A Z
A Z
C Y
B Y
A X
A X
C X
A Z
B Y
B Y
B Z
B Y
A X
C X
A X
A Z
A X
A X
A Y
C X
A Z
A X
C Y
A Y
C X
B Y
B Y
A X
B Y
A X
B Y
C X
B X
A X
A Z
A X
B Y
A Y
B Y
C X
B X
C Z
B Y
B Y
B Y
A X
B Y
A Y
A Z
C Z
A Y
B Y
C Y
C Z
A X
A Y
B Y
A Y
B X
C X
C Z
B X
A Z
A Z
B Y
A Y
A Z
B Z
C X
A Z
C X
B X
B Y
A Y
B X
A Z
A Y
A X
A X
B Y
A Y
C X
B Y
A X
B Z
B X
B Y
A Y
B Y
B X
A Z
A X
A X
B Y
A Z
B X
A Y
C Z
A Y
A X
B Y
B X
A Z
C Z
B X
A Z
A Y
A Z
B Y
B Y
C X
B Y
A Y
A Y
A Y
A X
B Y
B Y
A Z
A X
B X
A X
B Y
B Y
A Y
A Y
A X
B Y
B Y
B Y
C Z
B Y
A X
C X
A X
C X
B X
A X
A X
B Y
B Y
A X
B Y
B Y
A X
A Z
A Y
B Y
A X
C Y
C X
B Y
B X
B Y
B Z
B Y
A Y
B Y
A Z
C Z
A Y
A Y
B Y
B Y
A X
B Y
B Y
A Y
B Y
B Y
A X
A Z
A Z
A Y
B Y
A X
A Z
B Y
A X
B X
C Y
A X
A Y
B X
A X
A Y
A Z
C X
A X
A X
B Y
B Y
B Y
A X
A Z
A X
B Y
B Y
C Y
C Z
B Y
B Y
B Y
C X
B Y
A Z
A X
B X
B Y
C Z
A Z
A Z
B Y
A Z
A X
A Z
B Y
A X
C X
B X
A Y
B Y
C Y
A X
A X
B Y
A Z
A X
A X
A Y
A Y
A Y
B Y
A Y
");
}
