use regex::Regex;
use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};

fn read_lines<P: AsRef<Path>>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // args[0] is name of program
    let pattern = Regex::new(&args[1]).unwrap();
    let path = Path::new(&args[2]);

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
