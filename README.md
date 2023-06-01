# SHA-1
The program implements a practical task: "Implementation of a Hashing Algorithm"
This project implements a custom hashing algorithm in Rust. The algorithm used is SHA-1 (Secure Hash Algorithm 1). It takes an input message and produces a 160-bit hash value known as a message digest.

Here is an example of the program output:

```
SHA-1 hash (SHA-1): c571b86549e49bf223cf648388c46288c2241b5a
--------------------------------------------------
Message: SHA-1
Expected hash: c571b86549e49bf223cf648388c46288c2241b5a
Computed hash: c571b86549e49bf223cf648388c46288c2241b5a
Test passed: true
--------------------------------------------------
Message: In cryptography, SHA-1 (Secure Hash Algorithm 1) is a hash function
Expected hash: 5106fcd6b2f5b09cc7e6b2d76a93578d74ef4755
Computed hash: 5106fcd6b2f5b09cc7e6b2d76a93578d74ef4755
Test passed: true
--------------------------------------------------
Message: In cryptography, SHA-1 (Secure Hash Algorithm 1) is a hash function which takes an input and produces a 160-bit (20-byte) hash value known as a message digest – typically rendered as 40 hexadecimal digits. It was designed by the United States National Security Agency, and is a U.S. Federal Information Processing Standard.The algorithm has been cryptographically broken but is still widely used.
Expected hash: e85bf139b96daa8af5ef79d8825a57fe39335016
Computed hash: e85bf139b96daa8af5ef79d8825a57fe39335016
Test passed: true
--------------------------------------------------
My implementation: 0.000015 seconds
Library implementation: 0.000018 seconds
Performance difference (my program -  lib program): -0.000003 seconds

```

## Instructions

To use the hashing algorithm in your Rust project, follow these steps:

1. Add the following line to your `Cargo.toml` file:

   ```toml
   [dependencies]
 sha1 = "0.10.5" 
 
 hex = "0.4.3"

## Code Explanation

- `my_sha1_hash`: Implements the SHA-1 hashing algorithm, calculating the hash of the input message and returning it as a hexadecimal string.
- `test_single_block()`: Tests the `my_sha1_hash` function with a single block message and verifies the computed hash against an expected hash.
- `test_multiple_blocks()`: Tests the `my_sha1_hash` function with a multi-block message and verifies the computed hash against an expected hash.
- `test_multiple_chunks()`: Tests the `my_sha1_hash` function with a message that requires multiple chunks for hashing and verifies the computed hash against an expected hash.
- `test_performance()`: Measures the performance of the `my_sha1_hash` function compared to a library implementation of SHA-1, providing insights into the execution time difference.

## Running the Program on the site play.rust-lang.org.

To run your code on play.rust-lang.org, you can follow these steps:

Go to the website [play.rust-lang.org](https://play.rust-lang.org/) .

Clear the existing code on the page.

Paste your code into the empty field.

Click the "Run" button.

Your code will be compiled and executed in the output window on the right. The execution result will be displayed in that area.


## Running the Program on the computer

To run the program, execute the following steps:

1. Ensure you have the Rust programming language installed.

2. Open a terminal or command prompt.

3. Navigate to the directory containing the source code file.

4. Compile the program by running the command: `cargo build`.

5. Run the program by executing the command: `cargo run`.



## License

This program is released under the [MIT License](LICENSE).

[![made-with-rust](https://img.shields.io/badge/Made%20with-Rust-1f425f.svg)](https://www.rust-lang.org/)
