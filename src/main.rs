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
    // args[0] is name of program
    let args: Vec<String> = env::args().collect();

    // We want three input arguments
    if args.len() != 3 {
        panic!("Two input arguments expected!")
    }

    // Ensure we have a valid regex pattern in argv[1]
    let pattern_result = Regex::new(&args[1]);
    let pattern;
    match pattern_result {
        Ok(re) => pattern = re, // No problem
        Err(err) => panic!("Could not parse regex with pattern: {}\n\tDue to error: {}", &args[1], err),
    }

    let path = Path::new(&args[2]);

    // Make sure the file is found, if not panic!
    let lines_result = read_lines(path);
    let lines;
    match lines_result {
        Ok(l) => lines = l,
        Err(err) => panic!("Error when trying to read file at: {}\n\tDue to error: {}", &args[2], err),
    }

    // File read properly, go through lines
    for line in lines {
        // Lines read properly
        if let Ok(line_text) = line {
            if pattern.is_match(&line_text) {
                println!("{}", line_text);
            }
        }
    }
}
