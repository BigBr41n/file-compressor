extern crate flate2;

use flate2::write::GzEncoder;
use flate2::Compression;
use std::env;
use std::fs::File;
use std::io::{self, BufReader, copy};
use std::time::Instant;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <input_file> <output_file>", args[0]);
        std::process::exit(1);
    }

    let input_path = &args[1];
    let output_path = &args[2];

    // Open the input file
    let input_file = File::open(input_path).unwrap_or_else(|err| {
        eprintln!("Failed to open input file '{}': {}", input_path, err);
        std::process::exit(1);
    });

    let mut reader = BufReader::new(input_file);

    // Create the output file
    let output_file = File::create(output_path).unwrap_or_else(|err| {
        eprintln!("Failed to create output file '{}': {}", output_path, err);
        std::process::exit(1);
    });

    let mut encoder = GzEncoder::new(output_file, Compression::default());
    let start = Instant::now();

    // Copy data and compress
    copy(&mut reader, &mut encoder).unwrap_or_else(|err| {
        eprintln!("Failed during compression: {}", err);
        std::process::exit(1);
    });

    
    let output_file = encoder.finish().unwrap_or_else(|err| {
        eprintln!("Failed to finalize compression: {}", err);
        std::process::exit(1);
    });

    
    println!(
        "Source length: {} bytes",
        reader.get_ref().metadata().unwrap().len()
    );
    println!(
        "Output length: {} bytes",
        output_file.metadata().unwrap().len()
    );
    println!("Elapsed time: {:?}", start.elapsed());

    Ok(())
}
