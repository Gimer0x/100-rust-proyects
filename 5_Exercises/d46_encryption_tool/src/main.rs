// Day 46: Project: File Encryption Tool
// Build a CLI utility that encrypts and decrypts files using AES symmetric encryption. 
// You’ll use the aes and cipher crates to safely encode file contents—perfect for securing 
// sensitive files or building password-protected backups.
// + Aes256Ctr: Stream cipher using AES-256.
// + XOR-based encryption = same function for encrypt/decrypt.
// + Use of hex to input human-readable keys.
// You now have the core of a secure file locker, password-based vault, or encrypted file sync app.
use aes::Aes256;
use cipher::{KeyIvInit, StreamCipher};
use std::fs::{self};
use std::io::{self, Write};

type Aes256Ctr = ctr::Ctr64BE<Aes256>;
// Prevents identical plaintexts from producing identical ciphertexts
// Without an IV: same plaintext + same key = same ciphertext
// With an IV: same plaintext + same key + different IV = different ciphertext
// WARNING: This value should be computed randomly!
const IV: &[u8; 16] = b"uniqueinitvector"; // Initialization Vector (fixed for demo)

fn main() {
    println!("File Encryption Tool");
    println!("1. Encrypt file");
    println!("2. Decrypt file");

    let choice = prompt("Choose an option: ");
    
    match choice.as_str() {
        "1" => {
            let file = prompt("Enter path to file to encrypt: ");
            let key = prompt("Enter 32-byte key (hex): ");
            let out = prompt("Output file path: ");

            // unwrap(): Only in tests, examples, or when you're absolutely certain it won't fail
            // unwrap_or(): When you have a simple, cheap default value
            // unwrap_or_else(): When the default is expensive, needs context, or you want to handle the error case
            encrypt_file(&file, &key, &out).unwrap_or_else(|e| eprintln!("Error: {}", e));
        }
        "2" => {
            let file = prompt("Enter path to file to decrypt: ");
            let key = prompt("Enter 32-byte key (hex): ");
            let out = prompt("Output file path: ");

            decrypt_file(&file, &key, &out).unwrap_or_else(|e| eprintln!("Error: {}", e));
        }
        _ => println!("Invalid option")
    }
}

fn encrypt_file(path: &str, key_hex: &str, out_path: &str) -> io::Result<()>{
    let key = hex::decode(key_hex).expect("Invalid hex key");
    let mut data = fs::read(path)?;

    let mut cipher = Aes256Ctr::new_from_slices(&key, IV).unwrap();
    cipher.apply_keystream(&mut data);

    fs::write(out_path, data)?;
    println!("File encrypted to '{}'", out_path);

    Ok(())
}

fn decrypt_file(path: &str, key_hex: &str, out_path: &str) -> io::Result<()> {
    encrypt_file(path, key_hex, out_path) // Same as encrypt (symmetric XOR)
}

fn prompt(msg: &str) -> String {
    print!("{}", msg);
    io::stdout().flush().unwrap();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}