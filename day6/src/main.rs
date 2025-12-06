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
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let file_path = args.input;

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
        for ((ref mut num, &num_next), &sign) in numbers
            .iter_mut()
            .zip(next_numbers.iter())
            .zip(signs.iter())
        {
            println!("{num} {sign} {num_next}");
            let new_num = match sign {
                "*" => &mut (**num * num_next),
                "+" => &mut (**num + num_next),
                _ => panic!("Illegal operation"),
            };
            *num = new_num;
        }
    }

    let result: i64 = numbers.iter().sum();

    println!("part1 {result}");

    Ok(())
}
