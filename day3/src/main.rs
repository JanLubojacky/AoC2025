/// A key insight is that keeping a small integer before a greater one is never optimal
///
/// First implementation is a greedy algorithm and while it works its complexity is
/// O(n * k) (where k is the power limit) because it needs to scan the array for each limit
/// remaining
///
/// Second implementation uses a monotonic stack and runs in O(n)
///
use clap::Parser;
use std::cmp::PartialOrd;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    #[arg(short, long)]
    input: String,
    #[arg(short, long)]
    limit: u64,
}

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
///
/// note: this is tail recursion and would be better written as a loop
fn greedy_battery_tuner(battery_bank: &[u64], power_limit: u64) -> String {
    if power_limit == 0 {
        return String::new();
    }
    let remaining_limit = power_limit - 1;

    let (max_idx, max_value) =
        max_idx_and_val_from_slice(&battery_bank[0..battery_bank.len() - remaining_limit as usize])
            .expect("Bank is empty");

    return max_value.to_string()
        + &greedy_battery_tuner(&battery_bank[max_idx + 1..], remaining_limit).to_string();
}

fn monotonic_stack_battery_tuner(battery_bank: &[u64], power_limit: u64) -> String {
    let mut stack: Vec<u64> = Vec::new();

    for (idx, &val) in battery_bank.iter().enumerate() {
        let remains = battery_bank.len() - idx;
        let need = power_limit as usize - stack.len();
        let mut can_pop = remains.saturating_sub(need);

        // we can replace smaller values from earlier with larger ones from later if we still have
        // enough values left to create a digit of power_limit places
        while can_pop > 0 && stack.last().is_some_and(|&last| last > val) {
            can_pop -= 1;
            stack.pop();
        }

        // bank has space left we can push stuff inside
        if (stack.len() as u64) < power_limit {
            stack.push(val);
        }
    }

    stack.iter().map(|&num| num.to_string()).collect::<String>()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let file_path = args.input;
    let power_limit = args.limit;

    let mut total_power = 0;

    let file =
        File::open(&file_path).map_err(|e| format!("Failed to open file {}: {}", file_path, e))?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;

        let battery_bank: Vec<u64> = line
            .chars()
            .filter_map(|ch| ch.to_digit(10))
            .map(|num| num as u64)
            .collect();

        // let power: u64 = greedy_battery_tuner(&battery_bank, power_limit).parse()?;
        let power: u64 = monotonic_stack_battery_tuner(&battery_bank, power_limit).parse()?;
        total_power += power;
    }

    println!("total power is {total_power}");

    Ok(())
}
