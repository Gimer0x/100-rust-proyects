// Day 83: Project: Encrypted Notes Manager (AES Secure CLI)
// Build a command-line notes manager that allows you to write and read encrypted
// notes using AES-256 encryption. You'll use aes, cbc, and hex crates to
// implement secure local storage.
// Key Concepts:
// + AES-256 encryption with aes + block-modes
// + Hex encoding for safe storage
// + Basic file I/O with BufReader and OpenOptions
// You now have a functional, secure CLI app that stores notes encrypted at rest.
// A strong foundation for password managers, secure journals, or encrypted chat logs.
use aes::Aes256;
use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use cbc::{Decryptor, Encryptor};
use std::fs::{self, OpenOptions};
use std::io::{self, BufRead, Write};

type Aes256CbcEnc = Encryptor<Aes256>;
type Aes256CbcDec = Decryptor<Aes256>;

const KEY: &[u8; 32] = b"an_example_very_secure_key_32byt";
const IV: &[u8; 16] = b"unique_iv_16byte";

fn encrypt(plain_text: &str) -> String {
    let key = (*KEY).into();
    let iv = (*IV).into();
    let pt = plain_text.as_bytes();
    // AES block size 16; buffer must fit plaintext + PKCS7 padding
    let mut buf = vec![0u8; pt.len() + 16];
    buf[..pt.len()].copy_from_slice(pt);
    let ciphertext = Aes256CbcEnc::new(&key, &iv)
        .encrypt_padded_mut::<Pkcs7>(&mut buf, pt.len())
        .expect("encrypt");
    hex::encode(ciphertext)
}

fn decrypt(cipher_hex: &str) -> String {
    let key = (*KEY).into();
    let iv = (*IV).into();
    let mut bytes = hex::decode(cipher_hex).expect("Invalid hex");
    let decrypted = Aes256CbcDec::new(&key, &iv)
        .decrypt_padded_mut::<Pkcs7>(&mut bytes)
        .expect("Decryption failed");
    String::from_utf8_lossy(decrypted).to_string()
}

fn write_note() {
    print!("Enter your note: ");
    io::stdout().flush().unwrap();

    let mut note = String::new();
    io::stdin().read_line(&mut note).unwrap();
    let note = note.trim();

    let encrypted = encrypt(note);

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("notes.db")
        .expect("Cannot open file!");

    writeln!(file, "{}", encrypted).unwrap();
    println!("Note saved securely.");
}

fn read_notes() {
    println!("\n🔓 Decrypting all notes:\n");

    let file = fs::File::open("notes.db").expect("No notes found!");
    let reader = io::BufReader::new(file);

    for (i, line) in reader.lines().enumerate() {
        let encrypted = line.unwrap();
        let decrypted = decrypt(&encrypted);
        println!("{}. {}", i + 1, decrypted);
    }
}

fn main() {
    println!("Encrypted Notes Manager");
    println!("1) Write a note");
    println!("2) Read all notes");
    print!("Choose an option: ");
    io::stdout().flush().unwrap();

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();

    match choice.trim() {
        "1" => write_note(),
        "2" => read_notes(),
        _ => println!("Invalid option"),
    }
}
