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

#[derive(Debug)]
struct UnionFind {
    sizes: Vec<u64>,
    members: Vec<u64>,
    parents: Vec<usize>,
}

impl UnionFind {
    fn union(&mut self, u: usize, v: usize) {
        if self.sizes[u] < self.sizes[v] {
            self.parents[u] = v;

            if self.sizes[v] == 0 {
                self.sizes[v] = 1;
            } else {
                self.sizes[v] += self.sizes[u];
            }

            self.members[v] += self.members[u];
        } else {
            self.parents[v] = u;

            if self.sizes[u] == 0 {
                self.sizes[u] = 1;
            } else {
                self.sizes[u] += self.sizes[v];
            }

            self.members[u] += self.members[v];
        }
    }
    fn find(&self, id: usize) -> usize {
        if id == self.parents[id] {
            return id;
        } else {
            self.find(self.parents[id])
        }
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
    }

    let mut uf = UnionFind {
        sizes: vec![0; nodes.len()],
        members: vec![1; nodes.len()],
        parents: (0..nodes.len()).collect(),
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

    // for i in 0..1000.min(edges.len()) { // part 1
    for i in 0..edges.len() {
        // part 2
        let edge = &edges[i];

        // join nodes with this edge if they are already not together
        let root_u = uf.find(edge.u);
        let root_v = uf.find(edge.v);
        if root_u != root_v {
            uf.union(root_u, root_v);
        }

        // if everything is in one component break
        let mut parents = 0;
        for (i, &pn) in uf.parents.iter().enumerate() {
            if i == pn {
                parents += 1;
            }
        }
        if parents == 1 {
            part2 = nodes[edge.u].x as u64 * nodes[edge.v].x as u64;
            break;
        }
    }

    // uf.members.sort_by(|a, b| b.cmp(a));
    // println!("{:?}", uf.members);
    // println!("PART 1: {}", uf.members[0] * uf.members[1] * uf.members[2]);

    println! {"PART 2: {part2}"};

    Ok(())
}
