use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input: String,
    part: u8,
}

#[derive(Clone, Copy, Debug)]
enum Op {
    Plus,
    Times,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let file = File::open(&args.input)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

    match args.part {
        1 => println!("Part 1: {}", solve_part1(&lines)?),
        2 => println!("Part 2: {}", solve_part2(&lines)?),
        _ => {
            println!("Part 1: {}", solve_part1(&lines)?);
            println!("Part 2: {}", solve_part2(&lines)?);
        }
    }

    Ok(())
}

fn solve_part1(lines: &[String]) -> Result<i64, Box<dyn std::error::Error>> {
    let mut rev_lines = lines.iter().rev();
    let signs_line = rev_lines.next().ok_or("File is empty")?;
    let signs: Vec<Op> = signs_line
        .split_whitespace()
        .map(|ch| match ch {
            "*" => Op::Times,
            "+" => Op::Plus,
            _ => panic!("Invalid sign: {}", ch),
        })
        .collect();

    let first_line = rev_lines.next().ok_or("File has only one line")?;
    let mut numbers: Vec<i64> = first_line
        .split_whitespace()
        .filter_map(|num| num.parse().ok())
        .collect();

    for line in rev_lines {
        let next_numbers: Vec<i64> = line
            .split_whitespace()
            .filter_map(|num| num.parse().ok())
            .collect();

        for ((num, &num_next), &sign) in numbers
            .iter_mut()
            .zip(next_numbers.iter())
            .zip(signs.iter())
        {
            *num = match sign {
                Op::Times => *num * num_next,
                Op::Plus => *num + num_next,
            };
        }
    }

    Ok(numbers.iter().sum())
}

fn solve_part2(lines: &[String]) -> Result<i64, Box<dyn std::error::Error>> {
    // Find the maximum line length to create a uniform grid
    let max_len = lines.iter().map(|l| l.len()).max().unwrap_or(0);

    // Convert to a 2D character grid, padding shorter lines with spaces
    let grid: Vec<Vec<char>> = lines
        .iter()
        .map(|l| {
            let mut chars: Vec<char> = l.chars().collect();
            chars.resize(max_len, ' ');
            chars
        })
        .collect();

    let num_rows = grid.len();
    let num_cols = max_len;
    let data_rows = num_rows - 1; // Last row contains operators

    // Find separator columns (columns that are ALL spaces in data rows)
    let is_separator: Vec<bool> = (0..num_cols)
        .map(|col| (0..data_rows).all(|row| grid[row][col] == ' '))
        .collect();

    // Group adjacent non-separator columns into problems
    let mut problems: Vec<(Vec<usize>, Op)> = Vec::new();
    let mut current_cols: Vec<usize> = Vec::new();

    for col in 0..num_cols {
        if is_separator[col] {
            if !current_cols.is_empty() {
                let op = find_operator(&grid, data_rows, &current_cols)?;
                problems.push((current_cols.clone(), op));
                current_cols.clear();
            }
        } else {
            current_cols.push(col);
        }
    }
    // Don't forget the last problem if it doesn't end with a separator
    if !current_cols.is_empty() {
        let op = find_operator(&grid, data_rows, &current_cols)?;
        problems.push((current_cols, op));
    }

    // Solve each problem
    let total: i64 = problems
        .iter()
        .map(|(cols, op)| {
            // Each column forms a number by reading digits top-to-bottom
            let numbers: Vec<i64> = cols
                .iter()
                .filter_map(|&col| {
                    let digits: String = (0..data_rows)
                        .filter_map(|row| {
                            let c = grid[row][col];
                            if c.is_ascii_digit() { Some(c) } else { None }
                        })
                        .collect();

                    if digits.is_empty() {
                        None
                    } else {
                        Some(digits.parse::<i64>().unwrap())
                    }
                })
                .collect();

            match op {
                Op::Plus => numbers.iter().sum::<i64>(),
                Op::Times => numbers.iter().product::<i64>(),
            }
        })
        .sum();

    Ok(total)
}

fn find_operator(
    grid: &[Vec<char>],
    sign_row: usize,
    cols: &[usize],
) -> Result<Op, Box<dyn std::error::Error>> {
    for &col in cols {
        match grid[sign_row][col] {
            '+' => return Ok(Op::Plus),
            '*' => return Ok(Op::Times),
            _ => continue,
        }
    }
    Err("No operator found in problem columns".into())
}
