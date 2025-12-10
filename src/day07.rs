use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn run() -> io::Result<()> {
    let reader = BufReader::new(File::open("inputs/day07.txt")?);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));

    Ok(())
}

fn part1(lines: &[String]) -> u64 {
    let grid: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();
    let rows = grid.len();
    let cols = grid[0].len();

    let (sr, sc) = (0..rows)
        .flat_map(|r| (0..cols).map(move |c| (r, c)))
        .find(|&(r, c)| grid[r][c] == 'S')
        .expect("no S");

    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((sr + 1, sc));

    let mut count = 0u64;

    while let Some((r, c)) = queue.pop_front() {
        if r >= rows || c >= cols {
            continue;
        }
        if !visited.insert((r, c)) {
            continue;
        }

        match grid[r][c] {
            '.' => {
                queue.push_back((r + 1, c));
            }
            '^' => {
                if c > 0 {
                    queue.push_back((r + 1, c - 1));
                }
                if c + 1 < cols {
                    queue.push_back((r + 1, c + 1));
                }
                count += 1;
            }
            _ => {}
        }
    }

    count
}

fn part2(lines: &[String]) -> u128 {
    let grid: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();
    let rows = grid.len();
    let cols = grid[0].len();

    let (sr, sc) = (0..rows)
        .flat_map(|r| (0..cols).map(move |c| (r, c)))
        .find(|&(r, c)| grid[r][c] == 'S')
        .expect("no S");

    let mut counts = vec![vec![0u128; cols]; rows];
    let mut queue = VecDeque::new();

    if sr + 1 >= rows {
        return 1u128;
    }
    counts[sr + 1][sc] = 1;
    queue.push_back((sr + 1, sc));

    let mut finished: u128 = 0;

    while let Some((r, c)) = queue.pop_front() {
        let n = counts[r][c];
        if n == 0 {
            continue;
        }
        counts[r][c] = 0;

        match grid[r][c] {
            '.' => {
                if r + 1 >= rows {
                    finished = finished.saturating_add(n);
                } else {
                    let target = (r + 1, c);
                    if counts[target.0][target.1] == 0 {
                        queue.push_back(target);
                    }
                    counts[target.0][target.1] = counts[target.0][target.1].saturating_add(n);
                }
            }

            '^' => {
                if c == 0 {
                    finished = finished.saturating_add(n);
                } else {
                    let target = (r, c - 1);
                    if counts[target.0][target.1] == 0 {
                        queue.push_back(target);
                    }
                    counts[target.0][target.1] = counts[target.0][target.1].saturating_add(n);
                }

                if c + 1 >= cols {
                    finished = finished.saturating_add(n);
                } else {
                    let target = (r, c + 1);
                    if counts[target.0][target.1] == 0 {
                        queue.push_back(target);
                    }
                    counts[target.0][target.1] = counts[target.0][target.1].saturating_add(n);
                }
            }

            _ => {
                if r + 1 >= rows {
                    finished = finished.saturating_add(n);
                } else {
                    let target = (r + 1, c);
                    if counts[target.0][target.1] == 0 {
                        queue.push_back(target);
                    }
                    counts[target.0][target.1] = counts[target.0][target.1].saturating_add(n);
                }
            }
        }
    }

    finished
}
