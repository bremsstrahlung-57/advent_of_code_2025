use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn run() -> io::Result<()> {
    let reader = BufReader::new(File::open("inputs/day03.txt")?);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));

    Ok(())
}

fn part1(lines: &[String]) -> u64 {
    let mut sum = 0;

    for i in 0..lines.len() {
        let mut idx = 0;
        let num = &lines[i];
        let mut max = num.chars().nth(idx).unwrap().to_digit(10).unwrap() as u64;
        let num_len = num.len();

        for j in 1..num_len - 1 {
            if max == 9 {
                break;
            }

            let n = num.chars().nth(j).unwrap().to_digit(10).unwrap() as u64;

            if n > max {
                max = n;
                idx = j;
            }
        }

        let mut sec_max = num.chars().nth(idx + 1).unwrap().to_digit(10).unwrap() as u64;

        for k in idx + 1..num_len {
            if sec_max == 9 {
                break;
            }
            let n = num.chars().nth(k).unwrap().to_digit(10).unwrap() as u64;
            if n > sec_max {
                sec_max = n;
            }
        }

        let res = max * 10 + sec_max;
        sum += res;
    }

    sum
}

fn part2(lines: &[String]) -> u128 {
    let mut total: u128 = 0;
    let mut stack: Vec<char> = vec![];

    for i in 0..lines.len() {
        let num = &lines[i];
        let n = num.len();

        for (j, ch) in num.chars().enumerate() {
            if stack.len() == 0 {
                stack.push(ch);
                continue;
            }
            while let Some(&top) = stack.last() {
                if top < ch && (stack.len() - 1) + (n - j) >= 12 {
                    stack.pop();
                } else {
                    break;
                }
            }
            if stack.len() < 12 {
                stack.push(ch);
            }
        }

        let s = stack
            .iter()
            .fold(0u128, |acc, &c| acc * 10 + c.to_digit(10).unwrap() as u128);
        total += s;
        stack.clear();
    }

    total
}
