use clap::Parser;
use std::process::exit;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    #[arg(short, long)]
    input: String,
    part: u8,
}

#[derive(Clone, Copy)]
enum Sign {
    Plus,
    Times,
}

impl Sign {
    fn from(ch: &str) -> Self {
        match ch {
            "*" => Sign::Times,
            "+" => Sign::Plus,
            _ => panic!("Invalid sign!"),
        }
    }
}

fn part1(lines: &[String], signs: &[Sign]) -> Result<i64, Box<dyn std::error::Error>> {
    let mut rev_lines = lines.iter().rev();

    let initial_numbers = rev_lines.next().ok_or("File has only one line")?;
    let mut numbers: Vec<i64> = initial_numbers
        .split_whitespace()
        .filter_map(|num| num.parse().ok())
        .collect();

    // println!("{numbers:?}");

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
            // println!("{num} {sign} {num_next}");
            *num = match sign {
                Sign::Times => *num * num_next,
                Sign::Plus => *num + num_next,
            };
        }
    }

    let result: i64 = numbers.iter().sum();

    println!("part1 {result}");
    Ok(result)
}

fn print_mat(mat: &Vec<Vec<char>>) {
    println!("{}", "#".repeat(30));
    for row in mat {
        println!("{row:?}");
    }
    println!("{}", "#".repeat(30));
}

fn part2(lines: &[String], signs: &[Sign]) -> Result<i64, Box<dyn std::error::Error>> {
    // convert to a matrix of chars
    let mut char_matrix: Vec<Vec<char>> = lines.iter().map(|s| s.chars().collect()).collect();
    // print_mat(&char_matrix);

    // transpose
    let mut new_char_matrix: Vec<Vec<char>> = vec![Vec::new(); char_matrix[0].len()];
    for row in char_matrix.iter() {
        for j in 0..row.len() {
            new_char_matrix[j].push(row[j]);
        }
    }
    char_matrix = new_char_matrix;
    //
    // print_mat(&char_matrix);

    let mut result = 0;
    let mut sign_iter = signs.iter();
    let mut sign = sign_iter.next().expect("No signs?");
    let mut acc = match sign {
        Sign::Plus => 0,
        Sign::Times => 1,
    };

    for row in char_matrix {
        let num = row.iter().collect::<String>().trim().parse::<u64>();
        match num {
            Ok(num) => {
                // dbg!(num);
                acc = match sign {
                    Sign::Plus => acc + num,
                    Sign::Times => acc * num,
                }
            }
            Err(_) => {
                // dbg!(acc);
                result += acc;
                sign = sign_iter.next().expect("not enough signs?");
                acc = match sign {
                    Sign::Plus => 0,
                    Sign::Times => 1,
                }
            }
        }
    }
    result += acc;

    println!("part 2 {result}");

    Ok(0)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let file_path = args.input;
    let part = args.part;

    let file =
        File::open(&file_path).map_err(|e| format!("Failed to open file {}: {}", file_path, e))?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

    let mut rev_lines = lines.iter().rev();

    let signs = rev_lines.next().ok_or("File is empty")?;
    let signs: Vec<Sign> = signs.split_whitespace().map(|ch| Sign::from(ch)).collect();

    // The lines without the last line (which contains the signs)
    let lines_without_signs: Vec<String> = lines.iter().take(lines.len() - 1).cloned().collect();

    if part == 1 {
        part1(&lines_without_signs, &signs)?;
    } else if part == 2 {
        part2(&lines_without_signs, &signs)?;
    }

    Ok(())
}
