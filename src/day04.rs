use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::vec;

pub fn run() -> io::Result<()> {
    let reader = BufReader::new(File::open("inputs/day04.txt")?);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));

    Ok(())
}

const OFFSETS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn part1(lines: &[String]) -> i32 {
    let mut count = 0;

    for i in 0..lines.len() {
        let row = lines[i].as_bytes();

        for j in 0..row.len() {
            if row[j] != b'@' {
                continue;
            }

            let mut neigh = 0;

            for (di, dj) in OFFSETS {
                let ni = i as i32 + di;
                let nj = j as i32 + dj;

                if ni < 0 || nj < 0 {
                    continue;
                }
                let ni = ni as usize;
                let nj = nj as usize;
                if ni >= lines.len() || nj >= row.len() {
                    continue;
                }

                if lines[ni].as_bytes()[nj] == b'@' {
                    neigh += 1;
                }
            }

            if neigh < 4 {
                count += 1;
            }
        }
    }

    count
}

fn part2(lines: &[String]) -> i32 {
    let mut grid = lines
        .iter()
        .map(|s| s.as_bytes().to_vec())
        .collect::<Vec<_>>();
    let mut queue: Vec<(i32, i32)> = vec![];

    loop {
        let prev_state = queue.len();

        for i in 0..grid.len() {
            let row = grid[i].clone();
            for j in 0..row.len() {
                if row[j] != b'@' {
                    continue;
                }

                let mut neigh = 0;

                for (di, dj) in OFFSETS {
                    let ni = i as i32 + di;
                    let nj = j as i32 + dj;

                    if ni < 0 || nj < 0 {
                        continue;
                    }
                    let ni = ni as usize;
                    let nj = nj as usize;
                    if ni >= grid.len() || nj >= row.len() {
                        continue;
                    }

                    if grid[ni][nj] == b'@' {
                        neigh += 1;
                    }
                }

                if neigh < 4 {
                    grid[i][j] = b'.';
                    queue.push((i as i32, j as i32));
                }
            }
        }
        if queue.len() == prev_state {
            break;
        } else {
            continue;
        }
    }

    queue.len() as i32
}
