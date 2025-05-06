use std::time::Instant;

use base64::{engine::general_purpose, Engine};
use blake3::Hasher;
use xp_blake3::Result;

fn main() -> Result<()> {
    let input = "hello world";

    // Start timing
    let start_time = Instant::now();

    // With update
    let mut hasher = Hasher::new();
    hasher.update(input.as_bytes());
    let hash = hasher.finalize();

    // End timing
    let duration = start_time.elapsed();

    let b58 = bs58::encode(hash.as_bytes()).into_string();
    let b64 = general_purpose::URL_SAFE_NO_PAD.encode(hash.as_bytes());

    // Prints
    println!("{:>12}: {}", "Hash Hex", hash.to_hex());                          // d74981efa70a0c880b8d8c1985d075dbcbf679b99a5f9914e5aaf96b831a9e24
    println!("{:>12}: {}", "Hash b58", b58);                                    // FVPfbg9bK7mj7jnaSRXhuVcVakkXcjMPgSwxmauUofYf
    println!("{:>12}: {}", "Hash b64", b64);                                    // 10mB76cKDIgLjYwZhdB128v2ebmaX5kU5ar5a4ManiQ
    println!("{:>12}: {}", "Hash Len", hash.as_bytes().len());                  // 32
    println!("{:>12}: {:.3?} ms", "Duration", duration.as_secs_f64() * 1000.0); // 0.014 ms

    Ok(())
}
