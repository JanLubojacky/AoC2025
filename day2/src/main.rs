use std::{fs, process::exit};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    #[arg(short, long)]
    input: String,
}

fn main() {
    let args = Args::parse();
    let file_path = args.input;
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let sum = contents.trim().split(",").fold(0, |acc, substring| {
        println!("Processing {substring}");

        let mut contents = substring.split("-");
        let start: usize = contents
            .next()
            .unwrap()
            .parse()
            .expect("Failed to parse start");

        let end: usize = contents
            .next()
            .unwrap()
            .parse()
            .expect("Failed to parse end");

        println!("start {start} end {end}");

        acc + (start..=end).into_iter().fold(0, |inner_acc, i| {
            let string_num = i.to_string();

            if string_num.len() % 2 == 1 {
                println!("odd {i}");
                return inner_acc;
            }

            println!("even {i}");

            let (half1, half2) = string_num.split_at(string_num.len() / 2);
            if half1 == half2 {
                println!("{half1} and {half2} are the same!");
                return inner_acc + i;
            }

            inner_acc
        })
    });

    println!("{sum}")
}
