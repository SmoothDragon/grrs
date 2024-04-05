use::std::io::{BufRead, BufReader};
use clap::Parser;
use itertools::Itertools;

#[derive(Parser)]
struct Cli{
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    
    let reader = BufReader::new(std::fs::File::open(args.path).expect("open failed"));
    let result = reader
        .lines()
        .flatten()
        .filter(|s| s.contains(&args.pattern))
        .join("\n")
        ;
    println!("{}", result)
    // for line in reader.lines() {
        // if line.contains(&args.pattern) {
            // println!("{}", &line);
        // }
    // }

}
