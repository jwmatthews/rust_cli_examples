use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn count(file: Box<dyn BufRead>) -> (usize, usize, usize) {
    let mut lines = 0;
    let mut words = 0;
    let mut bytes = 0;

    for line in file.lines() {
        let line = line.unwrap();
        lines += 1;
        words += line.split_whitespace().count();
        bytes += line.len();
    }

    (lines, words, bytes)
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: wc <filename>");
        std::process::exit(1);
    }

    for filename in &args[1..] {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);
        let (lines, words, bytes) = count(Box::new(reader));
        println!("{} {} {} {}", lines, words, bytes, filename);
    }

    Ok(())
}