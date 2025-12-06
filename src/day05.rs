use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn run() -> io::Result<()> {
    let reader = BufReader::new(File::open("inputs/day.txt")?);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));

    Ok(())
}

fn part1(lines: &[String]) -> u64 {
    0
}

fn part2(lines: &[String]) -> u64 {
    0
}
