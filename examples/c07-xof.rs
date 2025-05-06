use std::time::Instant;

use base64::{engine::general_purpose, Engine};
use blake3::Hasher;
use xp_blake3::Result;

const CUSTOM_OUTPUT_LEN: usize = 64;

fn main() -> Result<()> {
    let input = "hello world";

    // Start timing
    let start_time = Instant::now();

    // With update
    let mut hasher = Hasher::new();
    hasher.update(input.as_bytes());

    // Create a buffer for the custom-length output
    let mut custom_hash = vec![0u8; CUSTOM_OUTPUT_LEN];

    // Finalize in XOF mode to get an OutputReader
    let mut out_reader = hasher.finalize_xof();

    // Read the desired number of bytes from the OutputReader
    out_reader.fill(&mut custom_hash);

    // End timing
    let duration = start_time.elapsed();

    // Convert hash bytes to hex string for printing
    let hash_hex = hex::encode(&custom_hash);

    // Prints
    println!("{:>12}: {}...", "Hash Hex", &hash_hex[0..std::cmp::min(hash_hex.len(), 16)]);     // d74981efa70a0c88... - first 16 symbols
    println!("{:>12}: {}", "Hash Len", custom_hash.len());                                      // CUSTOM_OUTPUT_LEN
    println!("{:>12}: {:.3?} ms", "Duration", duration.as_secs_f64() * 1000.0);                 // 0.015 ms

    Ok(())
}
