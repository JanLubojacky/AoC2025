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

#[derive(Debug, Clone, Copy)]
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
        println!("lenght {lenght}");
        println!("width {width}");
        lenght * width
    }
}

// find two points with the largest manhattan distance between them
//
// - naive => calculate the manhattan distances between all pairs of points
// - can we do better?
//  - figure out the 4 points that are the closest to 0,0 / r_max,0 and 0,l_max, r_max, l_max i.e.
//  the four corners, the largest rectangle has to be made out of some combination of those points
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

    // now need to sort the points by different sorts, smallest x, smallest y, largest x, largest y
    // and find the combinations, we need to find two pairs of points
    //
    // 1: smallest x and smallest y & largest x and largest y
    // 2: largest x and smallest y & smallest x and largest y

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

    // println!("{:#?}", pts);

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

    // println!("{:#?}", pts);

    let diff1 = pts.first().ok_or("no pts")?.clone();
    let diff2 = pts.last().ok_or("no pts")?.clone();
    let area2 = diff1.area(diff2);

    println!("{area1} {area2}");

    println!("PART 1: {}", area1.max(area2));

    Ok(())
}
