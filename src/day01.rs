use std::fs::File;
use std::io::{self, BufRead, BufReader};

static LEFT: char = 'L';

pub fn run() -> io::Result<()> {
    let reader = BufReader::new(File::open("inputs/day01.txt")?);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));

    Ok(())
}

fn part1(lines: &[String]) -> i64 {
    let mut turns: i64 = 0;
    let mut pos: i64 = 50;

    for i in lines {
        let (d, n) = get_data(i);

        if d == LEFT {
            pos = (pos - n) % 100;
            turns = turns + check_zero(pos);
        } else {
            pos = (pos + n) % 100;
            turns = turns + check_zero(pos);
        }
    }

    turns
}

fn part2(lines: &[String]) -> i64 {
    let mut turns: i64 = 0;
    let mut pos: i64 = 50;

    for i in lines {
        let (d, n) = get_data(i);

        if d == LEFT {
            let start = floor_div_100(pos - 1);
            let end = floor_div_100(pos - n - 1);

            turns += start - end;

            pos = (pos - n).rem_euclid(100);
        } else {
            turns += (pos + n) / 100;
            pos = (pos + n) % 100;
        }
    }
    turns
    // let new: i64;
    // if d == LEFT {
    //     new = pos - n;
    //     if new < 0 {
    //         if pos != 0 {
    //             turns += 1;
    //         }
    //         if new < -99 {
    //             turns += count_multiples_between(pos, new);
    //         }
    //     }
    //     pos = (pos - n).rem_euclid(100);
    //     turns += check_zero(pos);
    // } else {
    //     new = pos + n;
    //     if new > 100 {
    //         turns += 1;
    //         if new > 199 {
    //             turns += count_multiples_between(pos, new);
    //         }
    //     }
    //     pos = (pos + n).rem_euclid(100);
    //     turns += check_zero(pos);
    // }
}

fn get_data(input: &String) -> (char, i64) {
    let direction = input.chars().nth(0).unwrap();
    let number: i64 = input[1..].parse().unwrap();
    (direction, number)
}

fn check_zero(pos: i64) -> i64 {
    if pos == 0 { 1 } else { 0 }
}

// fn count_multiples_between(old_pos: i64, new_pos: i64) -> i64 {
//     let low = min(old_pos, new_pos);
//     let high = max(old_pos, new_pos);

//     let mut c = (high / 100) - (low / 100);
//     c
// }

fn floor_div_100(n: i64) -> i64 {
    if n >= 0 { n / 100 } else { (n - 99) / 100 }
}
