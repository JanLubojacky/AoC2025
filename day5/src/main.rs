/// We need to build a graph from paper rolls, connecting them to each other
/// afterwards it is simple to check how many neighbours a paper roll has
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
    let mut lines = reader.lines();
    let mut ranges: Vec<(u64, u64)> = Vec::new();

    // now need to figure out

    for line in lines
        .by_ref()
        .take_while(|l| l.as_ref().is_ok_and(|l| !l.is_empty()))
    {
        let line = line?;
        // split once because it will produce two halfs, split gives an iterator over all splits
        let (start, end) = line.split_once('-').unwrap();
        let start: u64 = start.parse()?;
        let end: u64 = end.parse()?;
        ranges.push((start, end));
    }

    let mut fresh = 0;
    for line in lines {
        let line = line?;
        let num: u64 = line.parse()?;

        for &(start, end) in &ranges {
            if num >= start && num <= end {
                fresh += 1;
                break;
            }
        }
    }

    println!("part1 {fresh}");

    // part2
    ranges.sort_unstable();

    let mut merged_ranges: Vec<(u64, u64)> = Vec::new();

    for (start, end) in ranges {
        if let Some(&(last_start, last_end)) = merged_ranges.last() {
            if start <= last_end {
                if end > last_end {
                    merged_ranges.pop();
                    merged_ranges.push((last_start, end));
                }
            } else {
                merged_ranges.push((start, end));
            }
        } else {
            merged_ranges.push((start, end));
        }
    }

    let mut part2 = 0;
    for (start, end) in merged_ranges {
        part2 += end - start + 1
    }

    println!("part2 {part2}");

    Ok(())
}
