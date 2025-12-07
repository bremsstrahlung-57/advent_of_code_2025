use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn run() -> io::Result<()> {
    let reader = BufReader::new(File::open("inputs/day05.txt")?);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));

    Ok(())
}

fn part1(lines: &[String]) -> u32 {
    let (given_fresh_ids_range, given_ids) = get_id_ranges(lines);
    let mut fresh_ids: Vec<u64> = Vec::new();

    for id in given_ids {
        for &(lower, upper) in &given_fresh_ids_range {
            if lower <= id && upper >= id {
                if !fresh_ids.contains(&id) {
                    fresh_ids.push(id);
                }
            }
        }
    }

    fresh_ids.len() as u32
}

fn part2(lines: &[String]) -> u64 {
    let (mut given_fresh_ids_range, _) = get_id_ranges(lines);
    given_fresh_ids_range.sort();
    let fresh_ids: Vec<(u64, u64)> = merge(given_fresh_ids_range);
    let mut total: u64 = 0;

    for (lower, upper) in &fresh_ids {
        total += (upper - lower) + 1;
    }

    total
}

fn get_id_ranges(lines: &[String]) -> (Vec<(u64, u64)>, Vec<u64>) {
    let mut fresh_ranges: Vec<(u64, u64)> = vec![];
    let mut all_ids: Vec<u64> = vec![];
    let mut parsing = true;

    for (_, j) in lines.iter().enumerate() {
        if j.trim().is_empty() {
            parsing = false;
            continue;
        }

        if parsing {
            let (a, b): (u64, u64) = {
                let (x, y) = j.split_once('-').unwrap();
                (x.parse().unwrap(), y.parse().unwrap())
            };

            fresh_ranges.push((a, b));
        } else {
            let a: u64 = j.parse().unwrap();
            all_ids.push(a);
        }
    }

    (fresh_ranges, all_ids)
}

fn merge(intervals: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    let mut merged: Vec<(u64, u64)> = vec![intervals[0]];

    for &current in &intervals[1..] {
        let last = merged.last_mut().unwrap();

        if current.0 <= last.1 {
            last.1 = std::cmp::max(last.1, current.1);
        } else {
            merged.push(current);
        }
    }

    merged
}
