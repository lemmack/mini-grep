use std::env;
use std::fs;
use std::process;
use std::error::Error; // Import the Error trait

// Define a struct to hold our configuration
struct Config {
    query: String,
    filename: String,
}

impl Config {
    // Associated function (like a static method) to build a Config
    // Takes a slice of strings (&[String]) representing the arguments
    // Returns a Result containing a Config on success, or a static string slice error message on failure
    fn build(args: &[String]) -> Result<Config, &'static str> {
        // Use if instead of checking length later
        if args.len() < 3 {
            return Err("Not enough arguments! Usage: mini-grep <query> <filename>");
        }

        // Still cloning here, which is fine for now
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

fn main() { // Changed signature back - we handle exit internally
    let args: Vec<String> = env::args().collect();

    // Call Config::build. Use 'unwrap_or_else' for cleaner error handling.
    let config = Config::build(&args).unwrap_or_else(|err| {
        // This code block runs if Config::build returns an Err
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1); // Exit the program
    });

    // Call run. Use 'if let Err' to check for errors from run.
    if let Err(e) = run(config) {
        // This code block runs if run returns an Err
        eprintln!("Application error: {}", e);
        process::exit(1); // Exit the program
    }

    // No explicit Ok(()) needed here as main doesn't return Result anymore
}

// run function takes ownership of the Config struct
// It also returns a Result, allowing use of '?' inside it
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Use '?' instead of .expect(). If fs::read_to_string fails,
    // the error will be converted into Box<dyn Error> and returned from run.
    let contents = fs::read_to_string(config.filename)?;

    // Iterate and search - logic remains the same
    for line in contents.lines() {
        if line.contains(&config.query) {
            println!("{}", line);
        }
    }

    Ok(())
}