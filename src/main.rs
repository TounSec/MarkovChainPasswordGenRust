use clap::Parser;
use markov_generator::{AddEdges, Chain};
use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader};
use std::process::exit;
use color_print::cprintln;

// Struct
#[derive(Parser, Debug)]
#[command(version = "1.0", author = "TounSec", about = "🔢Generates a random password with wordlist and Markov Chain method🔢")]
struct Args {
    /// Sets length of the password
    #[arg(short, long, default_value = "16")]
    length: usize,
    /// Specifies the path of the wordlist
    #[arg(short, long)]
    wordlist: String
}

// Const
const DEPTH: usize = 6;

fn generate_password(length: usize, wordlist: String) -> String
{
    let mut chain = Chain::new(DEPTH);
    let path = Path::new(&wordlist);

    if !path.exists() {
        eprintln!("❗Error : The path {:?} does not exist", &path);
        exit(1);
    }

    let file = File::open(&path).expect("❗Error : The file doesn't exist");
    let buffer = BufReader::new(file);

    for line in buffer.lines() {
        let line = line.unwrap();
        chain.add_all(line.chars(), AddEdges::Both);
    }

    let mut password = String::new();

    while password.len() < length {
        let generated = chain.generate()
            .collect::<String>();
        password += &generated;
    }

    password.truncate(length);
    password
}

fn main() {
    let args = Args::parse();
    let password = generate_password(args.length, args.wordlist);
    cprintln!("Generated password : <green>{}</green>", password);
}
