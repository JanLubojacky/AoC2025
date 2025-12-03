/// This is a brute force solution, it would be more elegant to generate the invalid ids in each
/// range instead
use clap::Parser;
use std::cmp::PartialOrd;
use std::error::Error;
use std::{
    fs::{self, File},
    io::{BufRead, BufReader},
};

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    #[arg(short, long)]
    input: String,
}

// This would not work anyways, default probably doesn't make sense? Would make more sense to
// return None if the slice is empty and set max_val to the first element if it is not but at that
// point it writing it this way with the forloop seems less ergonomic than the max_by with custom
// ordering
// fn max_idx_and_val_from_slice<T: PartialOrd + Default + Copy>(arr: &[T]) -> (usize, T) {
//     let (mut max_idx, mut max_val): (usize, T) = (0, T::default());
//     for (idx, val) in arr.iter().enumerate() {
//         if *val > max_val {
//             max_idx = idx;
//             max_val = *val;
//         }
//     }
//
//     return (max_idx, max_val);
// }

fn max_idx_and_val_from_slice<T: PartialOrd + Clone>(arr: &[T]) -> Option<(usize, T)> {
    arr.iter()
        .enumerate()
        .max_by(|a, b| {
            if a.1 > b.1 {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Less
            }
        })
        .map(|(idx, val)| (idx, val.clone()))
}

/// battery_bank is the array of batteries
/// power_limitter is the number of batteries that should be turned on
/// returns the maximum power this array can give given the limitter
fn battery_tuner(battery_bank: Vec<u32>, power_limitter: u32) -> u32 {
    let (first_idx, first_value) =
        max_idx_and_val_from_slice(&battery_bank[0..battery_bank.len() - 1])
            .expect("Bank is empty");

    let (_, second_value) = max_idx_and_val_from_slice(&battery_bank[first_idx + 1..])
        .expect("First index should not be last index in the array");

    let result: u32 = (first_value.to_string() + &second_value.to_string())
        .parse()
        .expect("This should work");

    result
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let file_path = args.input;
    let file =
        File::open(&file_path).map_err(|e| format!("Failed to open file {}: {}", file_path, e))?;
    let reader = BufReader::new(file);

    let mut total_power = 0;
    for line in reader.lines() {
        let line = line?;

        let battery_bank: Vec<u32> = line.chars().filter_map(|ch| ch.to_digit(10)).collect();

        total_power += battery_tuner(battery_bank);
    }

    println!("total power is {total_power}");

    Ok(())
}
