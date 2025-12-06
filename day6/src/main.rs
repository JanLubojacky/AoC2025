use clap::Parser;
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

fn part1(file_path: String) -> Result<(), Box<dyn std::error::Error>> {
    let file =
        File::open(&file_path).map_err(|e| format!("Failed to open file {}: {}", file_path, e))?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

    let mut rev_lines = lines.iter().rev();

    let signs = rev_lines.next().ok_or("File is empty")?;
    let signs: Vec<&str> = signs.split_whitespace().collect();

    println!("{signs:?}");

    let initial_numbers = rev_lines.next().ok_or("File has only one line")?;
    let mut numbers: Vec<i64> = initial_numbers
        .split_whitespace()
        .filter_map(|num| num.parse().ok())
        .collect();

    println!("{numbers:?}");

    while let Some(line) = rev_lines.next() {
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
                "*" => *num * num_next,
                "+" => *num + num_next,
                _ => panic!("Illegal operation"),
            };
        }
    }

    let result: i64 = numbers.iter().sum();

    println!("part1 {result}");

    Ok(())
}

fn part2(file_path: String) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let file_path = args.input;
    let part = args.part;

    if part == 1 {
        part1(file_path);
    } else if part == 2 {
        part2(file_path);
    }

    Ok(())
}
