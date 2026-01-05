/// [x] parse in the input into a graph
/// [ ] get topological ordering of the graph
use clap::Parser;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    #[arg(short, long)]
    input: String,
}

/// needs to be implemented via recursion so that a node gets added only after all its children are
/// processed
///
/// we can also make some assumptions given the input
/// - we have the starting node of the DAG, so we will be able to reach everything we are interested in from it
fn topological_ordering(
    adj_list: &HashMap<String, Vec<String>>,
    starting_vertices: &[String],
) -> Vec<String> {
    let mut visited: HashSet<String> = HashSet::new();
    let mut topo: Vec<String> = Vec::new();

    for start_vertex in starting_vertices {
        topo_dfs_helper(start_vertex, &adj_list, &mut visited, &mut topo);
    }

    topo.reverse();
    topo
}

fn topo_dfs_helper(
    vertex: &str,
    adj_list: &HashMap<String, Vec<String>>,
    visited: &mut HashSet<String>,
    topo: &mut Vec<String>,
) {
    if visited.contains(vertex) {
        return;
    }
    visited.insert(vertex.to_string());
    if let Some(neighbours) = adj_list.get(vertex) {
        for n in neighbours {
            topo_dfs_helper(n, adj_list, visited, topo);
        }
    }

    topo.push(vertex.to_string());
}

/// graph should be a DAG so a bfs should finish eventually even without visited
/// keep visited count for each vertex
fn part1(adj_list: &HashMap<String, Vec<String>>) -> Result<u64, Box<dyn std::error::Error>> {
    let queue = vec!["you".to_string()];

    let topo = topological_ordering(adj_list, &queue);
    let mut visited_counts: HashMap<String, u64> = HashMap::new();
    visited_counts.insert("you".to_string(), 1);

    for v in topo {
        println!("topo: {v}");
        let current_visits = *visited_counts
            .get(&v)
            .expect("v has not been seen yet, topological ordering is wrong");

        if let Some(neighbours) = adj_list.get(&v) {
            for n in neighbours {
                *visited_counts.entry(n.to_string()).or_insert(0) += current_visits;
            }
        }
    }

    let result = visited_counts.get("out").ok_or("out unreachable")?;

    Ok(*result)
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

    let stack = vec!["start".to_string()];
    let result = part1(&adj_list)?;
    println!("PART 1: {result}");

    // let result = find_all_dac_fft_paths(&adj_list)?;
    // println!("PART 2: {result}");

    Ok(())
}
