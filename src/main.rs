use sha1::{Digest, Sha1};
use std::time::Instant;

/// Computes the SHA-1 hash of the given message and returns it as a hexadecimal string
fn my_sha1_hash(message: &[u8]) -> String {
    // Initialize variables h0, h1, h2, h3, h44
    let mut h0: u32 = 0x67452301;
    let mut h1: u32 = 0xEFCDAB89;
    let mut h2: u32 = 0x98BADCFE;
    let mut h3: u32 = 0x10325476;
    let mut h4: u32 = 0xC3D2E1F0;

    // Prepare the message
    let original_length = message.len() as u64;
    let bit_length = original_length * 8;

    //   Append the bit "1" to the message
    let mut padded_message = message.to_vec();
    padded_message.push(0x80);

    // Append "0" bits until the length is a multiple of 512 bits
    while (padded_message.len() * 8) % 512 != 448 {
        padded_message.push(0x00);
    }

    // Append the length of the original message in binary format
    padded_message.extend(&bit_length.to_be_bytes());

    // Divide the message into 512-bit blocks
    for chunk in padded_message.chunks(64) {
        let mut words = [0u32; 80];

        // Divide the block into 16 words of 32 bits each
        for (i, chunk) in chunk.chunks_exact(4).enumerate() {
            words[i] = u32::from_be_bytes(chunk.try_into().unwrap());
        }

        // Expand the words to 80 words
        for i in 16..80 {
            let temp = words[i - 3] ^ words[i - 8] ^ words[i - 14] ^ words[i - 16];
            words[i] = temp.rotate_left(1);
        }

        // Initialize temporary variables
        let mut a = h0;
        let mut b = h1;
        let mut c = h2;
        let mut d = h3;
        let mut e = h4;

        //    Loop from 0 to 79
        for i in 0..80 {
            let (f, k) = match i {
                0..=19 => ((b & c) | ((!b) & d), 0x5A827999),
                20..=39 => (b ^ c ^ d, 0x6ED9EBA1),
                40..=59 => ((b & c) | (b & d) | (c & d), 0x8F1BBCDC),
                60..=79 => (b ^ c ^ d, 0xCA62C1D6),
                _ => unreachable!(),
            };

            let temp = a
                .rotate_left(5)
                .wrapping_add(f)
                .wrapping_add(e)
                .wrapping_add(k)
                .wrapping_add(words[i]);
            e = d;
            d = c;
            c = b.rotate_left(30);
            b = a;
            a = temp;
        }

        // Update values of h0, h1, h2, h3, h4
        h0 = h0.wrapping_add(a);
        h1 = h1.wrapping_add(b);
        h2 = h2.wrapping_add(c);
        h3 = h3.wrapping_add(d);
        h4 = h4.wrapping_add(e);
    }

    // Concatenate h0, h1, h2, h3, h4 into a single 160-bit hash value
    let hash = [
        h0.to_be_bytes(),
        h1.to_be_bytes(),
        h2.to_be_bytes(),
        h3.to_be_bytes(),
        h4.to_be_bytes(),
    ]
        .concat();

    // Return the hash in hexadecimal format
    hex::encode(hash)
}

/// The main entry point of the program
fn main() {
    //Example
    println!("SHA-1 hash (SHA-1): {}", my_sha1_hash("SHA-1".as_bytes()));
    println!("--------------------------------------------------");
    test_single_block();
    test_multiple_blocks();
    test_multiple_chunks();
    test_performance();
}

/// Tests the SHA-1 hash function on a single block message and verifies the computed hash
fn test_single_block() {
    let message = "SHA-1";
    let expected_hash = "c571b86549e49bf223cf648388c46288c2241b5a";
    let hash = my_sha1_hash(message.as_bytes());
    println!("Message: {}", message);
    println!("Expected hash: {}", expected_hash);
    println!("Computed hash: {}", hash);
    println!("Test passed: {}", hash == expected_hash);
    println!("--------------------------------------------------");
}

/// Tests the SHA-1 hash function on a message that spans multiple blocks and verifies the computed hash
fn test_multiple_blocks() {
    let message = "In cryptography, SHA-1 (Secure Hash Algorithm 1) is a hash function";
    let expected_hash = "5106fcd6b2f5b09cc7e6b2d76a93578d74ef4755";
    let hash = my_sha1_hash(message.as_bytes());
    println!("Message: {}", message);
    println!("Expected hash: {}", expected_hash);
    println!("Computed hash: {}", hash);
    println!("Test passed: {}", hash == expected_hash);
    println!("--------------------------------------------------");
}

/// Tests the SHA-1 hash function on a message that is divided into multiple chunks and verifies the computed hash
fn test_multiple_chunks() {
    let message = "In cryptography, SHA-1 (Secure Hash Algorithm 1) is a hash function which takes an input and produces a 160-bit (20-byte) hash value known as a message digest – typically rendered as 40 hexadecimal digits. It was designed by the United States National Security Agency, and is a U.S. Federal Information Processing Standard.The algorithm has been cryptographically broken but is still widely used.";
    let expected_hash = "e85bf139b96daa8af5ef79d8825a57fe39335016";
    let hash = my_sha1_hash(message.as_bytes());
    println!("Message: {}", message);
    println!("Expected hash: {}", expected_hash);
    println!("Computed hash: {}", hash);
    println!("Test passed: {}", hash == expected_hash);
    println!("--------------------------------------------------");
}

/// Measures the performance of the SHA-1 hash function implementation and compares it to a library implementation
fn test_performance() {
    let message = "In cryptography, SHA-1 (Secure Hash Algorithm 1) is a hash function";

    // Measuring the execution time of our implementation
    let start = Instant::now();
    let _ = my_sha1_hash(message.as_bytes());
    let duration_my = start.elapsed().as_secs_f64();

    // Measuring the execution time of the library implementation
    let start = Instant::now();
    let mut hasher = Sha1::default();
    hasher.update(message.as_bytes());
    let _ = hasher.finalize();
    let duration_lib = start.elapsed().as_secs_f64();

    println!("My implementation: {:.6} seconds", duration_my);
    println!("Library implementation: {:.6} seconds", duration_lib);
    println!(
        "Performance difference (my program -  lib program): {:.6} seconds",
        duration_my - duration_lib
    );
}
