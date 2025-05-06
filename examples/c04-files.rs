use std::{fs::File, io::{self, BufReader}, path::Path, time::{Duration, Instant}};

use base64::{engine::general_purpose, Engine};
use blake3::Hasher;
use xp_blake3::Result;
use tracing::{info, warn, error};
use uuid::Uuid;

fn main() -> Result<()> {
    xp_blake3::init();
    println!();
    
    let files = &[
        // ed12566e434224cf6bae29b4e36bd152aee162e6f233499b7639b7513bafa1c4     0.016 ms        1269 bytes
        "examples/c03-keyed-double-uuids.rs",   // Small file                   
        // 7e47ccd0f39a47ce66296696167b03342c85171b3b1176bc886411e741bc590b     about 230 ms    3.5e8 bytes
        ".tmp/big-file.mp4",                    // Large file       
    ];

    for file in files{
        // skip if not found
        if !Path::new(file).exists(){
            warn!("File not found, skipping {}", file);
            continue;
        }

        info!("Processing file '{}'", file);
        match hash_file(file) {
            Ok((size, hash_hex, duration)) => {
                info!("     Size: {} bytes", size);
                info!("     Hash: {}", hash_hex);
                info!("Hash took: {:.3?} ms", duration.as_secs_f64() * 1000.0);
            }
            Err(e) => {
                error!("Error processing file {}: {}", file, e)
            }
        }

        println!();
    }

    Ok(())
}

fn hash_file(path: impl AsRef<Path>) -> Result<(u64, String, Duration)>{
    let path = path.as_ref();

    // Open file
    let file = File::open(path)?;

    // Get file size from metadata
    let file_size = file.metadata()?.len();

    // Create a buffered reader for efficient reading
    let mut reader = BufReader::new(file);
    
    // Start timing
    let start_time = Instant::now();

    let mut hasher = Hasher::new();

    // Use io::copy to pipe the reader directly into the hasher (which is impl io::Write)
    io::copy(&mut reader, &mut hasher);

    // Finalize the hash
    let hash = hasher.finalize();

    // End timing
    let duration = start_time.elapsed();

    // Convert hash to hex string
    let hash_hex = hash.to_hex().to_string();

    Ok((file_size, hash_hex, duration))
}
