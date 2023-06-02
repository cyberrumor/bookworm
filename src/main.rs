use std;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Collect the arguments that were passed to this via CLI
    let args: Vec<String> = std::env::args()
        .collect();

    // An argument wasn't received, except perhaps the name
    // of this executable.
    if args.len() <= 1 {
        println!("usage: bookworm <filename>");
        println!("Consider reading The Castle of Otranto:");
        println!("https://www.gutenberg.org/files/696/696-0.txt");
        std::process::exit(1);
    }

    // Get the filename that the user passed in.
    let filename: &str = &args[1];
    println!("filename: {}", filename);


    // Read it into a String.
    let corpus = std::fs::read_to_string(filename)?;

    // Just print the value of corpus.
    println!("{}", corpus);
    Ok(())
}


