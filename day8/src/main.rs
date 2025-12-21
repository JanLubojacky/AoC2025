use clap::Parser;
use itertools::{self, Itertools};
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

#[derive(Debug)]
struct UnionFind {
    ranks: Vec<u64>,
    parents: Vec<usize>,
    num_components: usize,
}

impl UnionFind {
    fn union(&mut self, u: usize, v: usize) {
        if self.ranks[u] < self.ranks[v] {
            self.parents[u] = v;
        } else if self.ranks[u] > self.ranks[v] {
            self.parents[v] = u;
        } else {
            self.parents[v] = u;
            self.ranks[u] += 1;
        }
        self.num_components -= 1;
    }
    fn find(&mut self, id: usize) -> usize {
        if id != self.parents[id] {
            self.parents[id] = self.find(self.parents[id]);
        }
        self.parents[id]
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
        ((self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)) as f64
    }
}

#[derive(Debug)]
struct Edge {
    dist: f64,
    u: usize,
    v: usize,
}

impl Edge {
    fn new(coords: &[Coord], u: usize, v: usize) -> Self {
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

    // read in input
    for line in reader.lines() {
        let line = line?;
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
    }

    let mut uf = UnionFind {
        ranks: vec![0; nodes.len()],
        parents: (0..nodes.len()).collect(),
        num_components: nodes.len(),
    };

    // go trough all combinations of edges
    // and sort them
    let mut edges: Vec<Edge> = Vec::new();
    for i in 0..nodes.len() {
        for j in i + 1..nodes.len() {
            edges.push(Edge::new(&nodes, i, j))
        }
    }
    edges.sort_by(|a, b| a.dist.partial_cmp(&b.dist).unwrap());

    let mut part2: u64 = 0;

    // part 1
    // for i in 0..1000.min(edges.len()) {
    // part 2
    for i in 0..edges.len() {
        let edge = &edges[i];

        // join nodes with this edge if they are already not together
        let root_u = uf.find(edge.u);
        let root_v = uf.find(edge.v);
        if root_u != root_v {
            uf.union(root_u, root_v);
        }

        // part 2
        if uf.num_components == 1 {
            part2 = nodes[edge.u].x as u64 * nodes[edge.v].x as u64;
            break;
        }
    }

    // PART 1
    // ensure path compression everywhere
    for i in 0..nodes.len() {
        uf.find(i);
    }
    // get value counts on uf.parents and multiply the first 3
    // let counts = uf.parents.iter().counts();
    // let mut vals: Vec<usize> = counts.iter().map(|(&&k, &v)| v).collect();
    // vals.sort_by(|a, b| b.cmp(a));
    // let part1 = vals[0] * vals[1] * vals[2];
    // println! {"PART 1: {part1}"};

    println! {"PART 2: {part2}"};

    Ok(())
}
