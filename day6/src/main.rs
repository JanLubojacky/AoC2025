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
    let signs: Vec<&str> = signs.split_whitespace().collect();

    // println!("{signs:?}");
    if part == 1 {
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
                    "*" => *num * num_next,
                    "+" => *num + num_next,
                    _ => panic!("Illegal operation"),
                };
            }
        }

        let result: i64 = numbers.iter().sum();

        println!("part1 {result}");
    } else if part == 2 {
        let mut number_cols: Vec<Vec<Vec<char>>> = Vec::new();

        let number_of_digits = rev_lines.len();
        println!("number of digits {number_of_digits}");

        for line in rev_lines.rev() {
            let number_of_numbers = line.split_whitespace().collect::<Vec<&str>>().len();
            let numbers = line.chars().collect::<Vec<char>>();

            if number_cols.is_empty() {
                number_cols = vec![vec![Vec::new(); number_of_digits]; number_of_numbers];
            }

            for (i, num) in numbers.chunks(number_of_digits + 1).enumerate() {
                for j in 0..number_of_digits {
                    if num[j] != ' ' {
                        let position = number_of_digits - j - 1;
                        number_cols[i][position].push(num[j]);
                    }
                }
            }
        }

        // let result: i64 = number_cols
        //     .iter()
        //     .map(|col| {
        //         col.iter()
        //             .filter_map(|arr| arr.into_iter().collect::<String>().parse::<i64>().ok())
        //             .sum::<i64>()
        //     })
        //     .sum();

        println!("************");
        for n in number_cols {
            println!("{n:?}")
        }

        println!("{result}");
    }

    Ok(())
}
