/// [x] parse in the input into a graph
/// [ ] BFS on the graph and each time a vertex is reached add 1 to it
/// [ ] Result will be in the out vertex
use clap::Parser;
use std::{
    collections::{HashMap, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    #[arg(short, long)]
    input: String,
}

/// graph should be a DAG so a bfs should finish eventually even without visited
/// keep visited count for each vertex
fn find_all_paths(
    adj_list: HashMap<String, Vec<String>>,
) -> Result<u64, Box<dyn std::error::Error>> {
    let mut visited: HashMap<String, u64> = HashMap::new();
    visited.insert("you".to_string(), 1);

    let mut queue: VecDeque<String> = VecDeque::new();
    queue.push_back("you".to_string());

    // while queue not empty
    while let Some(vertex) = queue.pop_front() {
        *visited.entry(vertex.clone()).or_insert(0) += 1;
        // println!("Vertex {vertex}");

        // if this vertex has neighbours push them to queue
        if let Some(neighbours) = adj_list.get(&vertex) {
            for n in neighbours {
                queue.push_back(n.clone());
            }
        }
    }

    let result = visited.get("out").ok_or("Out unreachable")?;

    Ok(*result)
}

fn visited_counts(
    adj_list: &HashMap<String, Vec<String>>,
    queue: &mut VecDeque<String>,
) -> HashMap<String, u64> {
    let mut visited: HashMap<String, u64> = HashMap::new();

    while let Some(vertex) = queue.pop_front() {
        *visited.entry(vertex.clone()).or_insert(0) += 1;
        // println!("Vertex {vertex}");

        // if this vertex has neighbours push them to queue
        if let Some(neighbours) = adj_list.get(&vertex) {
            for n in neighbours {
                queue.push_back(n.clone());
            }
        }
    }

    visited
}

/// first part
///
/// treating srv and dac as you and out run the first part -> how many paths reach dac from srv
/// treating srv and fft as you and out run the first part -> how many paths reach fft from srv
///
/// using the visited number from before and resetting everything else, run the first part now
/// from dac to fft
/// from fft to dac
///
/// reset everything again and run the first part from fft to out and dac to out
fn find_all_dac_fft_paths(
    adj_list: HashMap<String, Vec<String>>,
) -> Result<u64, Box<dyn std::error::Error>> {
    Ok(0)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let file_path = args.input;

    let file =
        File::open(&file_path).map_err(|e| format!("Failed to open file {}: {}", file_path, e))?;
    let reader = BufReader::new(file);

    let mut adj_list: HashMap<String, Vec<String>> = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        let mut splits = line.split_whitespace();

        let mut vertex = splits.next().ok_or("Empty line")?;
        vertex = vertex.strip_suffix(":").expect("No : at the end of vertex");

        let to_vertices: Vec<String> = splits.map(|s| s.to_string()).collect();

        adj_list.insert(vertex.to_string(), to_vertices);
    }

    // let result = find_all_paths(adj_list)?;
    // println!("PART 1: {result}");

    Ok(())
}
