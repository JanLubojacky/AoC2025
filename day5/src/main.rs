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

    for line in lines
        .by_ref()
        .take_while(|l| l.as_ref().is_ok_and(|l| !l.is_empty()))
    {
        let line = line?;
        println!("{line}");
    }

    println!("#######");

    for line in lines {
        let line = line?;
        println!("{line}");
    }

    Ok(())
}
