use std::time::Instant;

use base64::{engine::general_purpose, Engine};
use blake3::Hasher;
use xp_blake3::Result;
use uuid::Uuid;

fn main() -> Result<()> {
    let input = "hello world";

    // Gen keys
    let id1 = Uuid::from_u128(128); // Just a random UUID
    let id2 = Uuid::from_u128(128); 

    let mut key = [0u8; blake3::KEY_LEN]; // Init a 32-byte array
    key[0..16].copy_from_slice(id1.as_bytes());     // Copy 1st UUID's bytes
    key[16..32].copy_from_slice(id2.as_bytes());    // Copy 2nd UUID's bytes

    // Start timing
    let start_time = Instant::now();

    // With update
    let mut hasher = Hasher::new_keyed(&key);
    hasher.update(input.as_bytes());
    let hash = hasher.finalize();

    // End timing
    let duration = start_time.elapsed();

    let b58 = bs58::encode(hash.as_bytes()).into_string();
    let b64 = general_purpose::URL_SAFE_NO_PAD.encode(hash.as_bytes());

    // Prints
    println!("{:>12}: {}", "Hash Hex", hash.to_hex());                          // f73dc17cf278278525f17d59c94c5c70219cd9b544ba0740ad69a19719f4ad06
    println!("{:>12}: {}", "Hash Len", hash.as_bytes().len());                  // 32
    println!("{:>12}: {:.3?} ms", "Duration", duration.as_secs_f64() * 1000.0); // 0.013 ms

    Ok(())
}
