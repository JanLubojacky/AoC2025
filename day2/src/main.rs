use clap::Parser;
use std::fs;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    #[arg(short, long)]
    input: String,
}

fn silly_pattern(s: String) -> bool {
    let half = s.len() / 2 + 1;
    let chars: Vec<char> = s.chars().collect();
    let mut retval = false;

    for chunk_size in 1..half {
        let mut chunked_chars = chars.chunks(chunk_size);
        retval = true;

        if let Some(first_chunk) = chunked_chars.next() {
            for chunk in chunked_chars {
                if first_chunk != chunk {
                    retval = false;
                    break;
                }
            }
        }
        if retval == true {
            return retval;
        }
    }

    retval
}

fn main() {
    let args = Args::parse();
    let file_path = args.input;
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let sum = contents.trim().split(",").fold(0, |acc, substring| {
        // println!("Processing {substring}");

        let mut contents = substring.split("-");
        let start: usize = contents
            .next()
            .unwrap()
            .parse()
            .expect("Failed to parse start");

        let end: usize = contents
            .next()
            .unwrap()
            .parse()
            .expect("Failed to parse end");

        // println!("start {start} end {end}");

        acc + (start..=end).into_iter().fold(0, |inner_acc, i| {
            let string_num = i.to_string();

            // PART 1
            // if string_num.len() % 2 == 1 {
            //     return inner_acc;
            // }
            // let (half1, half2) = string_num.split_at(string_num.len() / 2);
            // if half1 == half2 {
            //     return inner_acc + i;
            // }

            // PART 2
            // take first
            if silly_pattern(string_num) {
                // println!("{i} is silly!");
                return inner_acc + i;
            }

            inner_acc
        })
    });

    println!("{sum}")
}
