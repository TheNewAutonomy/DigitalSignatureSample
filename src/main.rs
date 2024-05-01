mod signing_modules;
use std::env;
use std::fs::File;
use std::io::Read;

/// Read the text file into memory, returning a io error if anything fails
fn read_file(file_name: &str) -> Result<String, std::io::Error>
{
    let mut f = File::open(file_name)?;
    let mut buffer = String::new();

    f.read_to_string(&mut buffer)?;
    Ok(buffer)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
            eprintln!("Usage: {} FILE", args[0]);
            eprintln!("Example: {} body.txt", args[0]);
            std::process::exit(1);
    }

    let input = read_file(&args[1]).expect("error reading file");

    let hashed_message = signing_modules::hasher::hash_message(&input);
    let _ = signing_modules::signer::sign_message(&hashed_message);
}