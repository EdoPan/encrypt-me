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

> **Example of operation and path:** <br>
> "Choose operation: 1 <br>
> "Insert directory path: ./test"

## Some context
### About AES GCM
The AES GCM encryption is performed using the [aes_gcm](https://docs.rs/aes-gcm/latest/aes_gcm/) crate.
### Limitations
* The script is able to perform the operations only on ```.txt``` files. Future updates will enable the script to perform the encryption on all types of file. 
* For MacOS users the ```.DS_STORE``` file in the target directory and all it's subdirectories cause errors during the operations. To avoid this errors make sure to remove them all.
* Directories with spaces in their names haven't been handled.
## Disclaimer
The author assumes no responsibility for unrecoverable files or misuse of the code.
