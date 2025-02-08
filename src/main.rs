use sha1::{Sha1, Digest};
use clap::Parser;
use hex;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Simple program to crack sha1 hashes from a wordlist
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Hash to crack
    #[arg(short, long)]
    sha1_hash: String,

    /// Path to wordlist file
    #[arg(short, long)]
    file: String,
}


fn main() {
    let args = Args::parse();
    let hash = args.sha1_hash;
    let file_path = args.file;
    
    let file = File::open(file_path).expect("Could not open file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let word = line.expect("Could not read line");
        let mut sha1 = Sha1::new();
        sha1.update(word.as_bytes());
        let hash_result = sha1.finalize();
        let hash_result_hex = hex::encode(hash_result);

        if hash_result_hex == hash {
            println!("Hash cracked! Word: {}", word);
            return;
        }
    }

    println!("Hash not found in wordlist.");
}
