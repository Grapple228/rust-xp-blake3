use std::time::Instant;

use base64::{engine::general_purpose, Engine};
use blake3::Hasher;
use xp_blake3::Result;
use uuid::Uuid;

fn main() -> Result<()> {
    let input = "hello world";

    // Gen keys
    let base_key_uuid = Uuid::from_u128(128);
    let key_material = base_key_uuid.to_string();
    let key: [u8; blake3::KEY_LEN] = *blake3::hash(key_material.as_bytes()).as_bytes();

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
    println!("{:>12}: {}", "Hash Hex", hash.to_hex());                          // dd13b21efd0552ddba06a33d3c5a88f605fa35b898ab8af8617054a7cadb0079
    println!("{:>12}: {}", "Hash Len", hash.as_bytes().len());                  // 32
    println!("{:>12}: {:.3?} ms", "Duration", duration.as_secs_f64() * 1000.0); // 0.003 ms

    Ok(())
}
