use clap::Parser;
use std::collections::VecDeque;
/// We need to build a graph from paper rolls, connecting them to each other
/// afterwards it is simple to check how many neighbours a paper roll has
use std::collections::{HashMap, HashSet};
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

    let mut part1 = 0;
    for (k, v) in adj_list.iter() {
        if v.len() < 4 {
            part1 += 1;
        }
    }

    println!("Part 1 {part1}");

    // part 2 in each iteration remove rolls that have < 4 neighbours
    // repeat until no rolls can be removed (this seems naive)

    // First pass: collect nodes to remove
    let mut to_visit: VecDeque<(usize, usize)> = adj_list
        .iter()
        .filter(|(_, v)| v.len() < 4)
        .map(|(k, _)| *k)
        .collect();

    let mut removed: HashSet<(usize, usize)> = HashSet::new();
    let mut in_queue: HashSet<(usize, usize)> = to_visit.iter().cloned().collect();

    while let Some(node_to_visit) = to_visit.pop_front() {
        if removed.contains(&node_to_visit) {
            continue;
        }
        in_queue.remove(&node_to_visit);

        if let Some(neighbours) = adj_list.get(&node_to_visit) {
            let mut valid_neighbours = 0;
            for n in neighbours {
                if !removed.contains(n) {
                    valid_neighbours += 1;
                }
            }
            if valid_neighbours < 4 {
                removed.insert(node_to_visit);
                for n in neighbours {
                    if !in_queue.contains(n) {
                        in_queue.insert(*n);
                        to_visit.push_back(*n);
                    }
                }
            }
        }
    }

    let part2 = removed.len();

    println!("Part 2 {part2}");

    Ok(())
}
