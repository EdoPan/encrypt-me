# Encrypt Me
A recursive file encryption script in Rust.
## How to run the script
In order to run the script you have to:
* Compile the project: ```cargo build```
* Run the script: ```cargo run```
* Follow the instructions

## Operations
The script allows to perform two operations:
* Encrypt: all files starting from the provided directory will be encrypted using AES-GCM.
* Decrypt: every file previously encrypted will be decrypted using the ```secrets.txt``` file.

The example below works also with **test** and **test/** as input.
> **Example of operation and path:** <br>
> "Choose operation: 1 <br>
> "Insert directory path: ./test"

The script needs to be executed in the same directory where is present the directory on which you want to perform the operations.
## Some context
### About AES GCM
The AES GCM encryption is performed using the [aes_gcm](https://docs.rs/aes-gcm/latest/aes_gcm/) crate.
### Limitations
* The script is able to perform the operations only on ```.txt``` files.
* The script delete each file after encryption using ```fs::remove_file()``` however files after deletion remains in memory for some time (i.e. a skilled forensic analyst can recover them easily).
## Disclaimer
The author assumes no responsibility for unrecoverable files or misuse of the code.