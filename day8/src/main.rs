// kruskal
//
// implement a graph structure
// find all the edges and sort them
// implement a union find
// after the required number of iterations find out the size of all the sets in union find and
// multiply the 3 largest ones
use clap::Parser;
use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;
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

// union find
// init each component as a different root each component stores pointer to parent and subtree size
// FIND -> climb from the vertex to the root and return that root
// UNION -> ru = FIND(u) and rv = FIND(v) if they differ attach the smaller component under the
// root of the bigger component
//
// need to be able to find each node by index
// so a vector of structs where each struct points to its parent or says I am root?

struct UFNode {
    size: u64,
    parent: Option<usize>, // idx of the node in the union find it points to or None if this is the
                           // root
}

impl UFNode {
    fn union(u: usize, v: usize) {}
    fn find(id: usize) -> usize {
        0
    }
}

#[derive(Debug)]
struct Coord {
    x: i64,
    y: i64,
    z: i64,
}

impl Coord {
    fn dist(&self, other: &Self) -> f64 {
        return (
            (
                (self.x - other.x).pow(2) +
                (self.y - other.y).pow(2) +
                (self.z - other.z).pow(2)
            ) as f64
        ).sqrt();
    }
}

#[derive(Debug)]
struct Edge {
    dist: f64,
    u: usize,
    v: usize,
}

impl Edge {
    fn new(coords: &Vec<Coord>, u: usize, v: usize) -> Self {
        return Self {
            u: u,
            v: v,
            dist: coords[u].dist(&coords[v]),
        };
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let file_path = args.input;

    let file =
        File::open(&file_path).map_err(|e| format!("Failed to open file {}: {}", file_path, e))?;
    let reader = BufReader::new(file);

    // construct sorted edge list
    let mut nodes: Vec<Coord> = Vec::new();
    let mut uf: Vec<UFNode> = Vec::new();

    // read in input
    for line in reader.lines() {
        let line = line?;
        println!("{line}");
        let nums: Vec<i64> = line
            .split(',')
            .filter_map(|num| num.parse::<i64>().ok())
            .collect();
        if nums.len() != 3 {
            panic!("Invalid input line {line}!");
        }
        nodes.push(Coord {
            x: nums[0],
            y: nums[1],
            z: nums[2],
        });
        uf.push(UFNode {
            parent: None,
            size: 0,
        });
    }

    println!("{nodes:#?}");

    // go trough all combinations of edges
    // and sort them
    let mut edges: Vec<Edge> = Vec::new();
    for i in 0..nodes.len() {
        for j in i + 1..nodes.len() {
            edges.push(Edge::new(&nodes, i, j))
        }
    }
    edges.sort_by(|a, b| a.dist.partial_cmp(&b.dist).unwrap());

    // first few edges correct
    // for i in 0..4 {
    //     println!(
    //         "{:#?} {:#?} {:#?}",
    //         edges[i].dist, nodes[edges[i].u], nodes[edges[i].v]
    //     );
    // }

    Ok(())
}
