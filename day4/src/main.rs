/// We need to build a graph from paper rolls, connecting them to each other
/// afterwards it is simple to check how many neighbours a paper roll has
mod graph;
use clap::Parser;
use std::cmp::PartialOrd;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::graph::Graph;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    #[arg(short, long)]
    input: String,
}

const ROLL: char = '@';
const EMPTY: char = '.';

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let file_path = args.input;

    let file =
        File::open(&file_path).map_err(|e| format!("Failed to open file {}: {}", file_path, e))?;
    let reader = BufReader::new(file);

    let mut input_matrix: Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        input_matrix.push(line.chars().collect());
    }

    let mut adj_list: Vec<Vec<usize>> = Vec::new();

    for (i, vec) in input_matrix.iter().enumerate() {}

    // for (i, line) in reader.lines().enumerate() {
    //     let line = line?;
    //     for (j, symbol) in line.chars().enumerate() {
    //         // add nodes to the graph
    //         match symbol {
    //             '@' => {
    //                 // insert the new node
    //                 adj_list.push(Vec::new());
    //                 // check neighbours and add edges if we have them
    //                  for r in -1..1 {
    //                     for c in -1..1 {
    //                         if (r, c) == (0, 0) {
    //                             break;
    //                         }
    //                     }
    //                 }
    //             }
    //             '.' => {
    //                 // do nothing
    //             }
    //             _ => panic!("Unexpected symbol"),
    //         }
    //     }
    // }

    Ok(())
}
