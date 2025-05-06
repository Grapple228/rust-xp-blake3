use std::{fs::File, io::{self, BufReader}, path::Path, time::{Duration, Instant}};

use base64::{engine::general_purpose, Engine};
use blake3::Hasher;
use xp_blake3::Result;
use sha2::Digest;
use tracing::{info, warn, error};
use uuid::Uuid;

fn main() -> Result<()> {
    xp_blake3::init();
    println!();
    
    let files = &[
        // 3dc45818e0a46f1070eaae36cdb472d752b4e3634b5aea20594f7fa4b5bc183d     0.065 ms           1269 bytes
        "examples/c03-keyed-double-uuids.rs",   // Small file                   
        // 6d3c9cee24a203cc6980f055a6f20395e48266b388f12da83e6b9b6f9ef2e472     about 14 seconds   3.5e8 bytes
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

    let mut hasher = sha2::Sha256::new();

    // Use io::copy to pipe the reader directly into the hasher (which is impl io::Write)
    io::copy(&mut reader, &mut hasher);

    // Finalize the hash
    let hash_bytes = hasher.finalize();

    // End timing
    let duration = start_time.elapsed();

    // Convert hash to hex string
    // There may be used data_encoding crate or a simple loop as below
    let mut hash_hex = String::with_capacity(hash_bytes.len() * 2);
    for byte in hash_bytes{
        hash_hex.push_str(&format!("{:02x}", byte));
    }

    Ok((file_size, hash_hex, duration))
}
