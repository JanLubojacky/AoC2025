use clap::Parser;
use std::{
    collections::{HashMap, HashSet},
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

fn visited_counts(
    adj_list: &HashMap<String, Vec<String>>,
    queue: &[String],
) -> HashMap<String, u64> {
    let topo = topological_ordering(adj_list, &queue);
    let mut visited_counts: HashMap<String, u64> = HashMap::new();
    for v in queue {
        visited_counts.insert(v.to_string(), 1);
    }

    for v in topo {
        let current_visits = *visited_counts
            .get(&v)
            .expect("v has not been seen yet, topological ordering is wrong");

        if let Some(neighbours) = adj_list.get(&v) {
            for n in neighbours {
                *visited_counts.entry(n.to_string()).or_insert(0) += current_visits;
            }
        }
    }

    visited_counts
}

/// graph should be a DAG so a bfs should finish eventually even without visited
/// keep visited count for each vertex
fn part1(adj_list: &HashMap<String, Vec<String>>) -> Result<u64, Box<dyn std::error::Error>> {
    let queue = vec!["you".to_string()];

    let vc = visited_counts(adj_list, &queue);

    let result = vc.get("out").ok_or("out unreachable")?;

    Ok(*result)
}

fn part2(adj_list: &HashMap<String, Vec<String>>) -> Result<u64, Box<dyn std::error::Error>> {
    // from svr
    let queue = vec!["svr".to_string()];
    let vc_start = visited_counts(adj_list, &queue);
    let svr2dac = vc_start.get("dac");
    let svr2fft = vc_start.get("fft");

    // from dac
    let queue = vec!["dac".to_string()];
    let vc_dac = visited_counts(adj_list, &queue);
    let dac2fft = vc_dac.get("fft");
    let dac2out = vc_dac.get("out");

    // from fft
    let queue = vec!["fft".to_string()];
    let vc_fft = visited_counts(adj_list, &queue);
    let fft2dac = vc_fft.get("dac");
    let fft2out = vc_fft.get("out");

    let mut result = 0;
    if let (Some(svr2dac), Some(dac2fft), Some(fft2out)) = (svr2dac, dac2fft, fft2out) {
        result += svr2dac * dac2fft * fft2out;
    }
    if let (Some(svr2fft), Some(fft2dac), Some(dac2out)) = (svr2fft, fft2dac, dac2out) {
        result += svr2fft * fft2dac * dac2out;
    }

    Ok(result)
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

    let result = part1(&adj_list)?;
    println!("PART 1: {result}");

    let result = part2(&adj_list)?;
    println!("PART 2: {result}");

    Ok(())
}
