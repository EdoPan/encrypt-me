use std::io;
use std::fs;
use std::path::Path;
use std::fs::OpenOptions;
use std::fs::{DirEntry, File};
use std::io::{BufRead, BufReader, Write};
use base64::{engine::general_purpose, Engine};
use aes_gcm::{
    aead::{generic_array::GenericArray, Aead, KeyInit, OsRng, Key, Nonce},
    Aes256Gcm, AeadCore};

fn handle_user_input() -> (i8, String) {
    println!("
d88888b d8b   db  .o88b. d8888b. db    db d8888b. d888888b   .88b  d88. d88888b 
88'     888o  88 d8P  Y8 88  `8D `8b  d8' 88  `8D `~~88~~'   88'YbdP`88 88'     
88ooooo 88V8o 88 8P      88oobY'  `8bd8'  88oodD'    88      88  88  88 88ooooo 
88~~~~~ 88 V8o88 8b      88`8b      88    88~~~      88      88  88  88 88~~~~~ 
88.     88  V888 Y8b  d8 88 `88.    88    88         88      88  88  88 88.     
Y88888P VP   V8P  `Y88P' 88   YD    YP    88         YP      YP  YP  YP Y88888P 
                                                                                                                                                           
---------------------------------------------------------------------------------
A recursive file encryption script in Rust by EdoPan.
Insert the path of the directory from where you would like to perform 
the encryption. Two operation are permetted: Encrypt and Decrypt. 
The encryption is performed using AES-GCM.
---------------------------------------------------------------------------------");
    println!("Choose operation: Encrypt (1), Decrypt (2)");
    print!("Choose operation: ");
    io::stdout().flush().unwrap();
    let mut o = String::new();
    io::stdin()
        .read_line(&mut o).expect("Failed to read operation");
    print!("Insert directory path: ");
    io::stdout().flush().unwrap();
    let mut d: String = String::new();
    io::stdin()
        .read_line(&mut d).expect("Failed to read directory path");
    let operation: i8 = o.to_string().replace("\n", "").parse().unwrap();
    let dir_input = d.replace("\n", "");
    (operation, dir_input)
}

fn perform_encryption(to_encrypt: DirEntry) {
    // Read plaintext file
    let plaintext_lines = fs::read_to_string(to_encrypt.path())
        .expect("Failed to read the file to operation");

    // Encrypt the content of the file
    let p_bytes = plaintext_lines.as_bytes();
    let k: GenericArray<u8, _> = Aes256Gcm::generate_key(OsRng);
    let cipher = Aes256Gcm::new(&k);
    let n = Aes256Gcm::generate_nonce(&mut OsRng);
    let ciphertext = cipher.encrypt(&n, p_bytes.as_ref())
        .expect("Failed to perform encryption");

    // Encode the content of the file using base64
    let engine = general_purpose::STANDARD_NO_PAD;
    let key = engine.encode(k);
    let nonce = engine.encode(n);

    // Create .enc file 
    let plain_to_enc = to_encrypt.path().to_str()
        .expect("Failed to perform .to_str()").to_string() + ".enc";
    let mut enc_file = File::create(plain_to_enc)
        .expect("Failed to create encrypted file");
    write!(enc_file, "{}", engine.encode(ciphertext))
        .expect("Failed to write to enc_file");

    // Remove plaintext file
    fs::remove_file(to_encrypt.path())
        .expect("Failed to delete the plaintext file");

    println!("Encrypted: {:#?}", to_encrypt.path());

    // Create a file with key-nonce in "secret.txt" for future decryption
    let mut secrets_file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("secrets.txt")
        .expect("Failed to open secrets.txt");
    writeln!(secrets_file, "{} {}", key, nonce)
        .expect("Failed to write to secrets.txt");
}

fn perform_decryption(secrets_vector: &(String, String), to_decrypt: DirEntry) {
    // Read encrypted file
    let encrypted_lines = fs::read_to_string(to_decrypt.path())
        .expect("Failed to read the file to decrypt");

    let (key, nonce) = secrets_vector;
    
    // Decode the key and the nonce from secrets vector
    let engine = general_purpose::STANDARD_NO_PAD;
    let k = engine.decode(&key)
        .expect("Failed to decode the key");
    let n = engine.decode(&nonce)
        .expect("Failed to decode the nonce");

    // Create the key and the nonce used for encryption
    let key = Key::<Aes256Gcm>::from_slice(&k);
    let nonce = Nonce::<Aes256Gcm>::from_slice(&n);
    let cipher = Aes256Gcm::new(key);

    // Decode the content of the file
    let ciphertext = engine.decode(&encrypted_lines)
        .expect("Failed to decode the encrypted content");
    let plaintext = cipher.decrypt(nonce, ciphertext.as_ref())
        .expect("Failed to perform decryption");

    // Create the plaintext file
    let plaintext_lines = String::from_utf8(plaintext)
        .expect("Failed to convert the content");
    let dec_decrypt = to_decrypt.path().to_str()
        .expect("Failed to perform .to_str()").to_string().replace(".enc", "");
    let mut dec_file = File::create(dec_decrypt)
        .expect("Failed to create decrypted file");
    write!(dec_file, "{}", plaintext_lines)
        .expect("Failed to write to dec_file");

    // Remove encrypted file
    fs::remove_file(to_decrypt.path())
        .expect("Failed to delete .enc file");

    println!("Decrypted: {:#?}", to_decrypt.path());
}

fn collect_files(dir_path: &Path) -> Vec<DirEntry> {
    let mut files = Vec::new();
    if dir_path.is_dir() {
        for entry in fs::read_dir(dir_path)
            .expect("Failed to read directory") {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    files.extend(collect_files(&path));
                } else {
                    files.push(entry);
                }
            }
        }
    }
    files
}

fn handle_operation(operation: bool, dir_path: &Path) {
    let entries = collect_files(dir_path);

    // true: encrypt / false: decrypt
    if operation {
        for entry in entries {
            perform_encryption(entry);
        }
    } else {

        let secrets_file_path: &Path = Path::new("./secrets.txt");
        let file = File::open(secrets_file_path)
            .expect("Failed to open secrets.txt");

        // Create a vector of (key, nonce) pair from secrets.txt
        let mut secrets: Vec<(String, String)> = Vec::new();
        for line in BufReader::new(file).lines() {
            let line = line.expect("Error");
            if let Some((key, nonce)) = line.split_once(' ') {
                secrets.push((key.to_string(), nonce.to_string()));
            }
        }
        // Check the number of (key, nonce) pair 
        if secrets.len() < entries.len() {
            println!("Not enough keys and nonces to decrypt all files.");
            return;
        }

        // Decrypt all files 
        for (i, entry) in entries.into_iter().enumerate() {
            perform_decryption(&secrets[i], entry);
        }

    // Remove secrets.txt file
    fs::remove_file(secrets_file_path)
        .expect("Failed to delete secrets.txt");
    }
}

fn main() {
    let res = handle_user_input();
    let dir_path = Path::new(&res.1);
    match res.0 {
        1 => handle_operation(true, dir_path),
        2 => handle_operation(false, dir_path),
        _ => println!("Operation not permitted")
    }
}