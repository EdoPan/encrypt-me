use std::path::Path;
use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm};

fn scan_environment(base_path: &Path) {
    for entry in base_path.read_dir().expect("Unable to read path") {
        if let Ok(entry) = entry {
            if entry.path().is_dir() {
                println!("This is a directory: {:?}", entry.path());
                scan_environment(&entry.path());
            } else {
                println!("This is a file: {:?}", entry.path());
            }
        }
    }
}

// Input: key, nonce and plaintext
// Output: key, nonce, ciphertext
fn perform_encryption(plaintext: &[u8]) {

    let key = Aes256Gcm::generate_key(OsRng);
    let cipher = Aes256Gcm::new(&key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng); // 96-bits; unique per message
    let ciphertext = cipher.encrypt(&nonce, plaintext.as_ref());
    
    match ciphertext {
        Ok(c) => {
            println!("Ciphertext: {:#?}", c);
        }
        Err(e) => {
            eprintln!("An error occured during encryption: {}", e);
        }
    }
}

/*
// Input: key, nonce and ciphertext
// Output: plaintext
fn perform_decryption() {
    match ciphertext {
        Ok(c) => {
            let plaintext = cipher.decrypt(&nonce, c.as_ref());
            println!("Plaintext: {:#?}", &plaintext);
        }
        Err(e) => {
            eprintln!("Decryption failed: {:?}", e);

        }
    }
}
*/


fn main() {
    // Select base bath
    //let base_path = Path::new("./test");
    //scan_environment(base_path);
    perform_encryption(b"test");
}

/*
TODO:
- perform_encryption needs to open, encrypt and close a file
- User CLI: choose base path, choose operation encryption/decryption
 */