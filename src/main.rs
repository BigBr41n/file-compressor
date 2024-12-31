extern crate flate2;

use flate2::write::GzEncoder;
use flate2::Compression;
use std::env;
use std::fs::File;
use std::io::{self, BufReader, copy};
use std::time::Instant;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("Usage: {} <input_file> <output_file> l[1,2,3]", args[0]);
        std::process::exit(1);
    }


    let input_path = &args[1];
    let output_path = &args[2];


    // get the compression level
    let compression = match args[3].as_str() {
        "l1" => Compression::new(1),
        "l2" => Compression::new(2),
        "l3" => Compression::new(3),
        _ => {
            eprintln!("Invalid compression level. Use 'l1', 'l2', or 'l3'");
            std::process::exit(1);
        }
    };

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

    let mut encoder = GzEncoder::new(output_file, compression);

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
