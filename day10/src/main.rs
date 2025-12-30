use clap::Parser;
use itertools::*;
use std::collections::HashSet;
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

// diagram can be a set of positions that need to be on
// buttons will also be sets of positions that will get turned on

fn num_of_combinations_with_replacement(n: u64, r: u64) -> u64 {
    let mut numerator = 1;
    let mut denominator = 1;
    for i in 0..r {
        numerator *= n + i;
        denominator *= i + 1;
    }
    numerator / denominator
}

fn solve_diagram(diagram: HashSet<u64>, buttons: Vec<HashSet<u64>>) -> u64 {
    // for all combinations with repetitions allowed but solutions without repetitions should be
    // explored first
    //
    // for a combination take the difference between all the sets to simulate the button toggling
    // and check if the result matches the diagram
    // compute the number of combinations for this lenght
    // iterate over them
    for r in 1..buttons.len() {
        for combination in buttons.iter().combinations_with_replacement(r) {
            let result = combination.iter().fold(HashSet::new(), |acc, &button| {
                acc.symmetric_difference(button).copied().collect()
            });

            if result == diagram {
                return r as u64;
            }
        }
    }

    0
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let file_path = args.input;

    let file =
        File::open(&file_path).map_err(|e| format!("Failed to open file {}: {}", file_path, e))?;
    let reader = BufReader::new(file);

    let mut result = 0;

    for line in reader.lines() {
        let line = line?;
        let mut splits: Vec<&str> = line.split_whitespace().collect();

        // first diagram
        let diagram: HashSet<u64> = splits[0]
            .replace('[', "")
            .replace(']', "")
            .chars()
            .enumerate()
            .filter_map(|(i, ch)| {
                if ch != '#' {
                    return None;
                }
                Some(i as u64)
            })
            .collect();

        // between buttons
        let buttons: Vec<HashSet<u64>> = splits[1..splits.len() - 1]
            .iter()
            .map(|&s| {
                s.replace('(', "")
                    .replace(')', "")
                    .split(',')
                    .filter_map(|s| s.parse::<u64>().ok())
                    .collect()
            })
            .collect();

        // last joltages
        let joltages: Vec<u64> = splits[splits.len() - 1]
            .replace('{', "")
            .replace('}', "")
            .split(',')
            .filter_map(|s| s.parse::<u64>().ok())
            .collect();

        result += solve_diagram(diagram, buttons);
    }

    println!("PART 1: {result}");

    Ok(())
}
