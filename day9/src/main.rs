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
        // println!("lenght {lenght}");
        // println!("width {width}");
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

    println!("{area1} {area2}");

    println!("PART 1: {}", area1.max(area2));

    Ok(())
}

#[derive(Debug, Copy, Clone)]
struct Rectangle {
    area: i64,
    first: Point,
    second: Point,
    third: Point,
    fourth: Point,
    fourth_in: u64,
}

fn up_left_down_right(p1: Point, p2: Point, p3: Point, p4: Point) -> (Point, Point) {
    let mut pts = [p1, p2, p3, p4];
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

    (pts[0], pts[3])
}

// if a red tile is always a corner it should be possible to check all triples that follow each
// other and mark its area and the location of the 4th corner (this should be the case based on
// the puzzle input)
//
// then do another round and for each rectangle formed by a triplet check if the 4th corner of
// another rectangle is inside it if yes, mark the rectangle as valid and track the area of
// valid rectangles, return the one with the largest area
//
// this does ignore thin rectangles formed by two successive points though but that should be a
// reasonable heuristic
fn part2(pts: Vec<Point>) -> Result<(), Box<dyn std::error::Error>> {
    let mut rects: Vec<Rectangle> = Vec::new();

    let mut extended_pts = pts.clone();
    extended_pts.extend_from_slice(&pts[..2]);

    for i in extended_pts.windows(3) {
        let first = i[0];
        let second = i[1];
        let third = i[2];
        let fourth = if first.x == second.x {
            Point {
                x: third.x,
                y: first.y,
            }
        } else if first.y == second.y {
            Point {
                x: first.x,
                y: third.y,
            }
        } else {
            panic!(
                "Invalid point configuration: {:?} {:?} {:?}",
                first, second, third
            );
        };

        println!("{first:?} {second:?} {third:?}");

        if first.x == third.x || first.y == third.y {
            panic!(
                "Three points on a line, this won't work :( {:?} {:?} {:?}",
                first, second, third
            );
        }

        // for a recatangle to be valid its 4th corner should be in at least two other rectangles
        //
        // is it possible that a 4th corner will be in 2 invalid rectangles?
        //
        rects.push(Rectangle {
            area: first.area(third),
            first: first,
            second: second,
            third: third,
            fourth: fourth,
            fourth_in: 0,
        });
    }

    for i in extended_pts.windows(3) {
        let first = i[0];
        let second = i[1];
        let third = i[2];
        let fourth = if first.x == second.x {
            Point {
                x: third.x,
                y: first.y,
            }
        } else if first.y == second.y {
            Point {
                x: first.x,
                y: third.y,
            }
        } else {
            panic!(
                "Invalid point configuration: {:?} {:?} {:?}",
                first, second, third
            );
        };

        for r in &mut rects {
            if r.fourth == fourth {
                continue;
            }
            let (ul, dr) = up_left_down_right(r.first, r.second, r.third, r.fourth);
            println!("ul {ul:?}, dr {dr:?}");
            if ul.x <= fourth.x && fourth.x <= dr.x && ul.y <= fourth.y && fourth.y <= dr.y {
                r.fourth_in += 1;
            }
        }
    }

    println!("{:#?}", rects);

    let max_rect = rects
        .iter()
        .filter(|&&r| r.fourth_in > 2)
        .max_by_key(|&&r| r.area)
        .ok_or("No rectangles")?;

    // 129411462 TOO LOW
    println!("PART2 {}", max_rect.area);

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

    // part1(pts.clone());
    part2(pts);

    Ok(())
}
