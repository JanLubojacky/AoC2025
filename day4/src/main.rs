use clap::Parser;
use std::cmp::PartialOrd;
/// We need to build a graph from paper rolls, connecting them to each other
/// afterwards it is simple to check how many neighbours a paper roll has
use std::collections::HashMap;
use std::panic;
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

    let mut adj_list: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();

    let sz_i = input_matrix.len();
    let sz_j = input_matrix[0].len();

    for (i, vec) in input_matrix.iter().enumerate() {
        for (j, ch) in vec.iter().enumerate() {
            match ch {
                '@' => {
                    // add node to adjacency list if not present
                    adj_list.entry((i, j)).or_insert(Vec::new());

                    for di in -1..=1 {
                        for dj in -1..=1 {
                            if di == 0 && dj == 0 {
                                continue;
                            }
                            let ni = i as i32 + di;
                            let nj = j as i32 + dj;
                            // bounds
                            if ni < 0 || nj < 0 || ni >= sz_i as i32 || nj >= sz_j as i32 {
                                continue;
                            }
                            // create edges to neighbours if they are in the graph
                            let (ni, nj) = (ni as usize, nj as usize);
                            let sym = input_matrix[ni][nj];
                            if input_matrix[ni][nj] == '@' {
                                adj_list.entry((i, j)).or_insert(Vec::new()).push((ni, nj));
                            }
                        }
                    }
                }
                '.' => {}
                _ => panic!("Unexpected symbol!"),
            }
        }
    }

    let mut free_rolls = 0;
    for (k, v) in adj_list.iter() {
        if v.len() < 4 {
            free_rolls += 1;
        }
    }

    println!("Part 1 {free_rolls}");

    // part 2 in each iteration remove rolls that have < 4 neighbours
    // repeat until

    Ok(())
}
