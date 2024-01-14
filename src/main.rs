use clap::Parser;
use rand::prelude::*;
use rand::distributions::{Distribution, Uniform};
use color_print::cprintln;

// Struct
#[derive(Parser, Debug)]
#[clap(version = "1.0", author = "TounSec", about = "🔢Generates a random password with Markov Chain method🔢")]
struct Args {
    /// Sets length of the password
    #[clap(short, long, default_value = "16")]
    length: usize
}

// Const
const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()_+";

// Generate password function
fn generate_password(length: usize) -> String
{
    let mut rng = thread_rng();
    let index = Uniform::from(0..CHARSET.len());

    (0..length)
        .map(|_| {
            let idx = index.sample(&mut rng);
            CHARSET[idx] as char
        })
        .collect()
}

fn main()
{
    let args = Args::parse();
    let password = generate_password(args.length);
    cprintln!("Generated password : <green>{}</green>", password);
}
