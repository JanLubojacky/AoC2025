use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct SafeDial {
    rotation: i32,
    positions: i32,
}

/// rotations left and right with wrapping the result bettween min and max
impl SafeDial {
    fn left(&mut self, by: usize) {
        self.rotation = (self.rotation - by as i32).rem_euclid(self.positions);
    }
    fn right(&mut self, by: usize) {
        self.rotation = (self.rotation + by as i32).rem_euclid(self.positions);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // parse input path as cli arg
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err("Missing input file argument".into());
    }
    let input_path = &args[1];

    let file =
        File::open(input_path).map_err(|e| format!("Failed to open file {}: {}", input_path, e))?;
    let reader = BufReader::new(file);

    let mut dial = SafeDial {
        rotation: 50,
        positions: 100,
    };

    let mut password = 0;

    // println!("Dial starts at {}", dial.rotation);

    // execute the rotation while keeping the state
    for line in reader.lines() {
        let line = line?;
        let mut chars = line.chars();

        let direction = chars
            .next()
            .ok_or("Line is empty, expected direction character")?;
        let number_string: String = chars.collect();
        let amount = number_string
            .parse::<usize>()
            .map_err(|e| format!("Failed to parse amount '{}': {}", number_string, e))?;

        match direction {
            'L' => dial.left(amount),
            'R' => dial.right(amount),
            _ => return Err(format!("Unexpected direction '{}'", direction).into()),
        }

        if dial.rotation == 0 {
            password += 1;
        }

        // println!("Dial is rotated {}, points at {}", line, dial.rotation)
    }

    println!("The password is {}", password);
    Ok(())
}
