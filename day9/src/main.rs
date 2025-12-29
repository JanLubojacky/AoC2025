use clap::Parser;
use std::{
    cmp::Ordering,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    #[arg(short, long)]
    input: String,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn l1(&self, other: Point) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
    fn area(&self, other: Point) -> i64 {
        let lenght = (self.x - other.x).abs() + 1;
        let width = (self.y - other.y).abs() + 1;
        lenght * width
    }
}

// find two points with the largest manhattan distance between them
//
// - naive => calculate the manhattan distances between all pairs of points
// - can we do better?
//  - there should be only 2 candidates for the largest rectangle
//
// 1: smallest x and smallest y & largest x and largest y
// 2: largest x and smallest y & smallest x and largest y
//
// find both with sorts and compare which one is larger
//
// O(n*log(n))
//
fn part1(mut pts: Vec<Point>) -> Result<(), Box<dyn std::error::Error>> {
    pts.sort_by(|a, b| {
        let a_sum = a.x + a.y;
        let b_sum = b.x + b.y;

        if a_sum < b_sum {
            return Ordering::Less;
        } else if a_sum > b_sum {
            return Ordering::Greater;
        }
        Ordering::Equal
    });

    let sum1 = pts.first().ok_or("no pts")?.clone();
    let sum2 = pts.last().ok_or("no pts")?.clone();
    let area1 = sum1.area(sum2);

    pts.sort_by(|a, b| {
        let a_diff = a.y - a.x;
        let b_diff = b.y - b.x;

        if a_diff < b_diff {
            return Ordering::Less;
        } else if a_diff > b_diff {
            return Ordering::Greater;
        }
        Ordering::Equal
    });

    let diff1 = pts.first().ok_or("no pts")?.clone();
    let diff2 = pts.last().ok_or("no pts")?.clone();
    let area2 = diff1.area(diff2);

    // println!("{area1} {area2}");

    println!("PART 1: {}", area1.max(area2));

    Ok(())
}

/// by counting edge crossings to the right of this point determine if inside or outside
fn is_point_inside(p: Point, vertical_edges: &Vec<(Point, Point)>) -> bool {
    let crossings: Vec<&(Point, Point)> = vertical_edges
        .iter()
        .filter(|(e1, e2)| {
            // (to the right) && (same height)
            if (e1.x >= p.x) && (e1.y <= p.y && p.y <= e2.y) {
                return true;
            }
            false
        })
        .collect();

    // println!(
    //     "point {:?}, n cross {}, cross {:?}",
    //     p,
    //     crossings.len(),
    //     crossings
    // );

    if crossings.len() % 2 == 0 {
        return false;
    }
    true
}

fn part2(pts: Vec<Point>) -> Result<(), Box<dyn std::error::Error>> {
    // collect all vertical edges of the polygon, two successive points form an edge
    let mut edges: Vec<(Point, Point)> = Vec::new();
    // zip each point with the next one (wrapping)
    for (p0, p1) in pts.iter().zip(pts.iter().cycle().skip(1)) {
        if p0.x == p1.x {
            // bounds checking assumes that the ordering in the tuple is (upper, lower)
            if p1.y < p0.y {
                edges.push((*p1, *p0));
            } else {
                edges.push((*p0, *p1));
            }
        }
    }

    let mut pl1: Point = Point { x: -1, y: -1 };
    let mut pl2: Point = Point { x: -1, y: -1 };
    let mut largest_area = 0;

    // for all unique combinations of 2 pts
    for i in 0..pts.len() {
        for j in (i + 1)..pts.len() {
            let c1 = pts[i];
            let c2 = pts[j];
            let c3 = Point { x: c1.x, y: c2.y };
            let c4 = Point { x: c2.x, y: c1.y };

            // println!("{c1:?} {c2:?} {c3:?} {c4:?}");

            if is_point_inside(c3, &edges) && is_point_inside(c4, &edges) {
                let area = ((c1.x - c2.x).abs() + 1) * ((c1.y - c2.y).abs() + 1);

                if area > largest_area {
                    pl1 = c1;
                    pl2 = c2;
                }
                // println!("Is inside area: {area}");
                largest_area = largest_area.max(area);
            }
        }
    }

    // 129411462 TOO LOW
    // 830804 TOO LOW
    // 4726851885 not right
    println!("PART 2: {}, p1 {:?}, p2 {:?}", largest_area, pl1, pl2);

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let file_path = args.input;

    let file =
        File::open(&file_path).map_err(|e| format!("Failed to open file {}: {}", file_path, e))?;
    let reader = BufReader::new(file);

    let mut pts: Vec<Point> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        // let (x: i64, y: i64) = line.split_once(',').map(|(a, b)| (a.parse(), b.parse()));
        let (a, b) = line.split_once(',').ok_or("no comma, invalid line!")?;
        let (x, y): (i64, i64) = (a.parse()?, b.parse()?);
        pts.push(Point { x: x, y: y })
    }

    part1(pts.clone());
    part2(pts);

    Ok(())
}
