use std::env;
use std::fs; // Import the file system module
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Not enough arguments! Usage: mini-grep <query> <filename>");
        process::exit(1);
    }

    let query = args[1].clone();
    let filename = args[2].clone();

    // We don't need to print these every time now
    // println!("Searching for '{}'", query);
    // println!("In file '{}'", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    // Iterate over each line in the file contents
    for line in contents.lines() {
        // Check if the line contains the query string
        if line.contains(&query) { // Pass query as a slice using &
            // Print the line if it matches
            println!("{}", line);
        }
    }
}