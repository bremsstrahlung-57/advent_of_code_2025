use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn run() -> io::Result<()> {
    let reader = BufReader::new(File::open("inputs/day02.txt")?);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));

    Ok(())
}

fn part1(lines: &[String]) -> u64 {
    let mut total = 0;
    let ranges = separate_ranges(lines);
    for (i, j) in ranges {
        for n in i..(j + 1) {
            let x = n.to_string();
            let len = x.len();
            if len % 2 != 0 {
                continue;
            }
            if x[0..len / 2] == x[len / 2..] {
                total += n;
            }
        }
    }
    total
}

fn part2(lines: &[String]) -> u64 {
    let mut total = 0;
    let ranges = separate_ranges(lines);
    for (i, j) in ranges {
        for n in i..=j {
            let x = n.to_string();
            let len = x.len();
            let mut invalid = false;
            for k in 1..=len / 2 {
                if len % k != 0 {
                    continue;
                }
                let prefix = &x[0..k];
                if prefix.repeat(len / k) == x {
                    invalid = true;
                    break;
                }
            }
            if invalid {
                total += n;
            }
        }
    }
    total
}

fn separate_ranges(input: &[String]) -> Vec<(u64, u64)> {
    let parsed = input.join(",");

    parsed
        .split(",")
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|r| {
            let (a, b) = r.split_once("-").unwrap();
            (
                a.trim().parse::<u64>().unwrap(),
                b.trim().parse::<u64>().unwrap(),
            )
        })
        .collect()
}
