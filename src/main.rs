use regex::Regex;
use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let pattern = Regex::new(&args[0]).unwrap();
    let path = Path::new(&args[1]);

    // File read properly
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            // Lines read properly
            if let Ok(line_text) = line {
                if pattern.is_match(&line_text) {
                    println!("{}", line_text);
                }
            }
        }
    }
}
