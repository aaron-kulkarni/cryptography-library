use std::process;
mod aes;
use aes::utils::AESConfig;
use aes::utils::KeyLength;
mod cli;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let config = match cli::init_aes_config() {
        Ok(x) => x,
        //TODO: exit gracefully
        Err(e) => process::exit(8),
    };

    if config.encrypt {
        println!("Encrypting...");
        let key_clone = config.key.clone();
        let mut text_clone = config.base_bytes.clone();
        let encrypted_bytes = aes::encrypt::encrypt(&mut text_clone, config.key);

        let mut file = match File::create("mykey.txt") {
            Ok(f) => f,
            Err(_) => {
                println!("Could not create mykey.txt.");
                process::exit(3);
            }
        };

        if let Err(_) = file.write_all(&key_clone) {
            println!("Failed to write to file mykey.txt.");
            process::exit(4);
        }

        let mut file2 = match File::create("mymsg.txt") {
            Ok(f) => f,
            Err(_) => {
                println!("Could not create mymsg.txt");
                process::exit(5);
            }
        };

        if let Err(_) = file2.write_all(&encrypted_bytes) {
            println!("Failed to write to mymsg.txt.");
            process::exit(6);
        }

        println!(
            "The key for this encryption has been saved in my_key.txt.
            The encrypted message has been saved in my_msg.txt.
            Doing another encryption operation will overwrite both these files."
        );

        return;
    } else {
        println!("Decrypting...");

        let decrypted_string = aes::decrypt::decrypt(config.base_bytes, config.key);
        println!("This is the decrypted string: {:?}", decrypted_string);
        return;
    }
}
