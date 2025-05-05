use std::fs;
use std::process;
use std::error::Error;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli{
    /// The pattern to look for
    #[arg(required = true)] // Explicitly mark as required
    query: String,

    /// The path to the file to search in
    #[arg(required = true, value_parser = clap::value_parser!(PathBuf))] // Use PathBuf for path type
    path: PathBuf,
}

fn main() {
    // Cli::parse() automatically handles argument parsing,
    // validation, help/version messages, and exits on error.
    let cli = Cli::parse();

    // If Cli::parse() returns, arguments are valid.
    // We pass the owned values from `cli` to `run`.
    if let Err(e) = run(cli.query, cli.path) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

// Update the run function signature to accept PathBuf
fn run(query: String, path: PathBuf) -> Result<(), Box<dyn Error>> {
    // fs::read_to_string works directly with PathBuf
    let contents = fs::read_to_string(path)?;

    // The rest of the function remains the same
    for line in contents.lines() {
        if line.contains(&query) { // Still need to borrow query for contains
            println!("{}", line);
        }
    }

    Ok(())
}