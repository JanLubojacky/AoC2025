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

    // process first line
    let first_line = lines.next().ok_or("Empty input?")??;
    let start_pos = first_line.find('S').ok_or("No start")?;

    // keep information about last line, as an array with positions and beam counts
    let mut beams = vec![0u64; first_line.len()];
    beams[start_pos] = 1;

    let mut splits: u64 = 0;

    // can probably manipulate the iterator to return every second item to skip empty lines
    for line in lines {
        let line = line?;
        // let mut new_beams = beams.clone();
        // let line_chars: Vec<char> = line.chars().collect();
        // let beamline: String = beams.iter().map(|b| b.to_string()).collect();
        // println!("{beamline}");
        // println!("{line}");
        for (i, _) in line.match_indices('^') {
            // if beam above this splitter, set it to 0 and increment neighbours
            if beams[i] != 0 {
                beams[i - 1] += beams[i];
                beams[i + 1] += beams[i];
                beams[i] = 0;
                splits += 1;
            }
        }
    }

    let timelines: u64 = beams.iter().sum();

    println!("SPLITS: {splits}");
    println!("TIMELINES: {timelines}");

    Ok(())
}
