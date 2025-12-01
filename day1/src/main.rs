use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

// For the first exercise I had 1007 which is correct
// before I had something that was too low
// 5829 TOO high
// 6219 TOO high

struct SafeDial {
    position: i32,
    number_of_positions: i32,
}

/// rotations left and right with wrapping the result bettween min and max
impl SafeDial {
    fn left(&mut self, by: i32) {
        self.position = (self.position - by).rem_euclid(self.number_of_positions)
    }

    fn right(&mut self, by: i32) {
        self.position = (self.position + by).rem_euclid(self.number_of_positions)
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
        position: 50,
        number_of_positions: 100,
    };

    let mut password = 0;

    println!("Dial starts at {}", dial.position);

    // execute the rotation while keeping the state
    for line in reader.lines() {
        let line = line?;
        let mut chars = line.chars();

        let direction = chars
            .next()
            .ok_or("Line is empty, expected direction character")?;
        let number_string: String = chars.collect();
        let mut amount = number_string
            .parse::<i32>()
            .map_err(|e| format!("Failed to parse amount '{}': {}", number_string, e))?;

        // handle cases when number of rotations is > the number of positions
        while amount >= dial.number_of_positions {
            amount -= dial.number_of_positions;
            println!("huge rotation +1");
            password += 1;
        }

        let prev_position = dial.position;

        // rotation to 0 from right couns twice

        match direction {
            'L' => {
                dial.left(amount);
                if (dial.position > prev_position) && (prev_position != 0) && (dial.position != 0) {
                    // we decreased the value so if we have a bigger value then before we had to
                    // pass trough zero, starting from 0 doesn't count as a pass
                    println!("PASS 0 BY LEFT ROT");
                    password += 1;
                }
            }
            'R' => {
                dial.right(amount);
                if (dial.position < prev_position) && (prev_position != 0) && (dial.position != 0) {
                    println!("PASS 0 BY RIGHT ROT");
                    password += 1;
                }
            }
            _ => return Err(format!("Unexpected direction '{}'", direction).into()),
        }

        if dial.position == 0 {
            println!("DIAL ENDED UP AT 0");
            password += 1;
        }

        println!("Dial is rotated {}, points at {}", line, dial.position)
    }

    println!("The password is {}", password);
    Ok(())
}
