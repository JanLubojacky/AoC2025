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

fn visited_counts(
    adj_list: &HashMap<String, Vec<String>>,
    queue: &mut VecDeque<String>,
) -> HashMap<String, u64> {
    let mut visited: HashMap<String, u64> = HashMap::new();

    while let Some(vertex) = queue.pop_front() {
        *visited.entry(vertex.clone()).or_insert(0) += 1;

        if let Some(neighbours) = adj_list.get(&vertex) {
            for n in neighbours {
                queue.push_back(n.clone());
            }
        }
    }

    visited
}

/// graph should be a DAG so a bfs should finish eventually even without visited
/// keep visited count for each vertex
fn find_all_paths(
    adj_list: &HashMap<String, Vec<String>>,
) -> Result<u64, Box<dyn std::error::Error>> {
    let mut queue: VecDeque<String> = VecDeque::new();
    queue.push_back("svr".to_string());

    let visited_counts = visited_counts(&adj_list, &mut queue);

    let result = visited_counts.get("out").ok_or("Out unreachable")?;

    Ok(*result)
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
    adj_list: &HashMap<String, Vec<String>>,
) -> Result<u64, Box<dyn std::error::Error>> {
    let mut queue: VecDeque<String> = VecDeque::new();
    queue.push_back("svr".to_string());

    let vc = visited_counts(adj_list, &mut queue);

    let result = vc.get("out").ok_or("Out unreachable")?;

    // let srv_to_fft = match vc.get("fft") {
    //     Some(v) => *v,
    //     None => 0,
    // };
    // let srv_to_dac = match vc.get("dac") {
    //     Some(v) => *v,
    //     None => 0,
    // };
    //
    // println!("srv_to_fft: {srv_to_fft}");
    // println!("srv_to_dac: {srv_to_dac}");
    //
    // queue = VecDeque::new();
    // queue.push_back("fft".to_string());
    // let vc = visited_counts(adj_list, &mut queue);
    // let fft_to_dac = match vc.get("dac") {
    //     Some(v) => *v,
    //     None => 0,
    // };
    //
    // println!("fft_to_dac: {fft_to_dac}");
    // let fft_to_out = match vc.get("out") {
    //     Some(v) => *v,
    //     None => 0,
    // };
    //
    // queue = VecDeque::new();
    // queue.push_back("dac".to_string());
    // let vc = visited_counts(adj_list, &mut queue);
    // let dac_to_fft = match vc.get("fft") {
    //     Some(v) => *v,
    //     None => 0,
    // };
    //
    // println!("dac_to_fft: {dac_to_fft}");
    // let dac_to_out = match vc.get("out") {
    //     Some(v) => *v,
    //     None => 0,
    // };
    // println!("fft_to_out: {fft_to_out}");
    // println!("dac_to_out: {dac_to_out}");
    //
    // let mut out = 0;
    // if srv_to_dac != 0 && dac_to_fft != 0 && fft_to_out != 0 {
    //     out += fft_to_out;
    // }
    // if srv_to_fft != 0 && fft_to_dac != 0 && dac_to_out != 0 {
    //     out += dac_to_out;
    // }
    //
    // Ok(out)
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

    let result = find_all_paths(&adj_list)?;
    println!("PART 1: {result}");

    // let result = find_all_dac_fft_paths(&adj_list)?;
    // println!("PART 2: {result}");

    Ok(())
}
