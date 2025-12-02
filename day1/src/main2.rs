use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/small").expect("Failed to read input");

    let mut pos = 50i32;
    let mut zero_finishes = 0;
    let mut zero_crossings = 0;

    for line in input.lines() {
        let (dir, dist_str) = line.split_at(1);
        let dist: i32 = dist_str.parse().unwrap();

        // Update position
        pos = match dir {
            "R" => {
                zero_crossings += (pos + dist) / 100 - pos / 100;
                pos + dist
            }
            "L" => {
                zero_crossings += (pos - 1).div_euclid(100) - (pos - dist - 1).div_euclid(100);
                pos - dist
            }
            _ => panic!("Unexpected instruction."),
        }
        .rem_euclid(100);

        // Part 1: count final positions at 0
        if pos == 0 {
            zero_finishes += 1;
        }
    }

    println!("Part 1: {zero_finishes}");
    println!("Part 2: {}", zero_finishes + zero_crossings);
}
