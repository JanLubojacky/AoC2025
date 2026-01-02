use clap::Parser;
use itertools::*;
use std::collections::{HashMap, HashSet};
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

/// Brute force iteration over all combinations
/// Since we want a pattern we need to find an intersection between the button presses that leads
/// to the patter, each button will be pressed either once or not at all since pressing a button
/// twice is equal to not pressing it at all and pressing it 3 times is equal to pressing it once
fn solve_diagram(diagram: HashSet<u64>, buttons: Vec<HashSet<u64>>) -> u64 {
    for r in 1..buttons.len() {
        for combination in buttons.iter().combinations(r) {
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

/// First try is a brute force algorithm, this will not work because we might need a large amount
/// of keypresses to get the required configuration
///
/// One way to solve this would be with ILP
///
/// Another would be an optimization solution where we first determine the resulting pattern from
/// even and odd joltages and then figure out which joltages can lead to that pattern, afterwards
/// we have to press those buttons some amount of times to get the result
///
/// Or we might be able to combine this
///
/// part 1: search for all combinations of buttons that would
/// lead to the pattern defined by the resulting joltage
///
/// part 2: once we have the buttons required to solve this if the matrix
/// is totally unimodular there will be an integer solution, we can iterate over all the candidates
/// and find one where the solution holds that x in (0,1,2,...)
fn balance_joltage(joltage: Vec<u64>, buttons: Vec<HashSet<u64>>) -> u64 {
    let mut r = 1;
    loop {
        for combination in buttons.iter().combinations_with_replacement(r) {
            let mut result = vec![0; joltage.len()];
            for &comb in &combination {
                for &value in comb {
                    result[value as usize] += 1;
                }
            }

            if result == joltage {
                println!("{:?}", combination);
                return r as u64;
            }
        }
        r += 1;
    }
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
        let splits: Vec<&str> = line.split_whitespace().collect();

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

        // result += solve_diagram(diagram, buttons);
        result += balance_joltage(joltages, buttons);
        // println!("{line} : {result}");
    }

    println!("SOLUTION: {result}");

    Ok(())
}
