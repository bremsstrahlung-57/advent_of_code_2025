use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn run() -> io::Result<()> {
    let reader = BufReader::new(File::open("inputs/day06.txt")?);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));

    Ok(())
}

fn part1(lines: &[String]) -> u64 {
    let grid = parse_grid(lines);
    let mut expr: Vec<u64> = Vec::new();
    let mut res: u64 = 0;
    let rows = grid.len();
    let cols = grid[0].len();

    for col in 0..cols {
        for row in 0..rows {
            let curr = &grid[row][col];
            if curr == "*" {
                res += expr.iter().map(|s| *s).product::<u64>();
                expr.clear();
            } else if curr == "+" {
                res += expr.iter().map(|s| *s).sum::<u64>();
                expr.clear();
            } else {
                let n: u64 = curr.parse().unwrap();
                expr.push(n);
            }
        }
    }

    res
}

fn parse_grid(lines: &[String]) -> Vec<Vec<String>> {
    lines
        .iter()
        .map(|line| line.split_whitespace().map(|s| s.to_string()).collect())
        .collect()
}

fn part2(lines: &[String]) -> u64 {
    let grid = transpose(parse_grid2(lines));
    let blocks = find_problem_blocks(&grid);
    let mut total: u64 = 0;

    for (l, r) in blocks {
        let (op, nums) = parse_blocks(&grid, l, r);
        total += eval_block(op, nums);
    }

    total
}

fn parse_grid2(lines: &[String]) -> Vec<Vec<char>> {
    let max_width = lines.iter().map(|l| l.len()).max().unwrap();

    lines
        .iter()
        .map(|l| {
            let padded = format!("{:<width$}", l, width = max_width);
            padded.chars().collect::<Vec<char>>()
        })
        .collect()
}

fn transpose(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut out = vec![vec![' '; rows]; cols];

    for r in 0..rows {
        for c in 0..cols {
            out[c][r] = grid[r][c];
        }
    }

    out
}

fn find_problem_blocks(grid: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let cols = grid.len();
    let mut blocks = Vec::new();
    let mut c = 0;

    while c < cols {
        if grid[c].iter().all(|&ch| ch == ' ') {
            c += 1;
            continue;
        }

        let start = c;

        while c < cols && !grid[c].iter().all(|&ch| ch == ' ') {
            c += 1;
        }

        blocks.push((start, c));
    }

    blocks
}

fn parse_blocks(grid: &Vec<Vec<char>>, l: usize, r: usize) -> (char, Vec<u64>) {
    let rows = grid[0].len();
    let op_row = rows - 1;
    let mut op_col = None;

    for c in l..r {
        let ch = grid[c][op_row];
        if ch == '+' || ch == '*' {
            op_col = Some(c);
            break;
        }
    }

    let op_col = op_col.expect("no operator in block");
    let op = grid[op_col][op_row];
    let mut nums = Vec::new();

    for c in l..r {
        let mut s = String::new();

        for r in 0..op_row {
            let ch = grid[c][r];
            if ch != ' ' {
                s.push(ch);
            }
        }

        if !s.is_empty() {
            nums.push(s.parse::<u64>().unwrap());
        }
    }

    (op, nums)
}

fn eval_block(op: char, nums: Vec<u64>) -> u64 {
    match op {
        '+' => nums.into_iter().sum(),
        '*' => nums.into_iter().product(),
        _ => unreachable!(),
    }
}
