use super::utils::AESConfig;
use crate::KeyLength;
use dialoguer::{Input, Select};
use rand::Rng;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::process;

pub fn init_aes_config() -> Result<AESConfig, Box<dyn Error>> {
    println!(
        "Good choice! Let's encrypt/decrypt your messages with the Advanced Encryption Standard."
    );

    let encrypt = match Select::new()
        .with_prompt("Do you want to encrypt or decrypt a message?")
        .default(0)
        .item("Encrypt")
        .item("Decrypt")
        .interact()
        .unwrap()
    {
        0 => true,
        1 => false,
        _ => panic!("how would this ever happen"),
    };

    if encrypt == true {
        let base_string: String = Input::new()
            .with_prompt("What string do you want to encrypt?")
            .interact_text()
            .unwrap();
        let key_length = match Select::new()
            .with_prompt("What length key do you want to use?")
            .default(0)
            .item(16)
            .item(24)
            .item(32)
            .interact()
            .unwrap()
        {
            0 => KeyLength::Len16,
            1 => KeyLength::Len24,
            2 => KeyLength::Len32,
            _ => KeyLength::Len16,
        };
        let use_own_key = match Select::new()
            .with_prompt("Do you want to use your own key or use a randomly generated one?")
            .default(0)
            .item("Randomly generated")
            .item("My own")
            .interact()
            .unwrap()
        {
            0 => false,
            1 => true,
            _ => false,
        };
        let kl = key_length.clone();
        if use_own_key == true {
            let provided_key = Input::new()
                .with_prompt(format!("Enter your {} byte key", kl as usize))
                .validate_with(|s: &String| -> Result<(), String> {
                    if s.len() != kl as usize {
                        let msg = format!(
                            "You gave a key made up of {} bytes. \
                            It should be {} bytes.",
                            s.len(),
                            kl as usize
                        );
                        Err(msg)
                    } else {
                        Ok(())
                    }
                })
                .interact_text()?;
            return Ok(AESConfig {
                base_bytes: base_string.into_bytes(),
                encrypt,
                key: provided_key.into_bytes(),
            });
        } else {
            let generated_key = key_generator(kl);
            return Ok(AESConfig {
                base_bytes: base_string.into_bytes(),
                encrypt,
                key: generated_key,
            });
        }
    } else {
        //decryption
        println!("Key is being read from aeskey.txt...");
        let mut key_file = File::open("aeskey.txt")?;
        let mut key: Vec<u8> = Vec::new();
        match key_file.read_to_end(&mut key) {
            Ok(_) => {}
            Err(_) => {
                println!(
                    "aeskey.txt does not exist. Are you sure you encrypted your files using AES?"
                )
            }
        }

        println!("Encrypted message is being read from aesmsg.txt...");
        let mut msg_file = File::open("aesmsg.txt")?;
        let mut base_bytes = Vec::new();

        match msg_file.read_to_end(&mut base_bytes) {
            Ok(_) => {}
            Err(_) => {
                println!(
                    "aesmsg.txt does not exist. Are you sure you encrypted your files using AES?"
                )
            }
        }

        return Ok(AESConfig {
            base_bytes,
            encrypt,
            key,
        });
    }
}

fn key_generator(key_length: KeyLength) -> Vec<u8> {
    let len = key_length as usize;
    let mut key = vec![0u8; len];
    rand::thread_rng().fill(&mut key[..]);
    return key;
}

pub fn run_aes(config: AESConfig) {
    if config.encrypt {
        println!("Encrypting...");
        let key_clone = config.key.clone();
        let mut text_clone = config.base_bytes.clone();
        let encrypted_bytes = super::encrypt::encrypt(&mut text_clone, config.key);

        let mut file = match File::create("aeskey.txt") {
            Ok(f) => f,
            Err(_) => {
                println!("Could not create aeskey.txt.");
                process::exit(1);
            }
        };

        if let Err(_) = file.write_all(&key_clone) {
            println!("Failed to write to file aeskey.txt.");
            process::exit(1);
        }

        let mut file2 = match File::create("aesmsg.txt") {
            Ok(f) => f,
            Err(_) => {
                println!("Could not create aesmsg.txt");
                process::exit(1);
            }
        };

        if let Err(_) = file2.write_all(&encrypted_bytes) {
            println!("Failed to write to aesmsg.txt.");
            process::exit(1);
        }

        println!(
            "The key for this encryption has been saved in aeskey.txt.
            The encrypted message has been saved in aesmsg.txt.
            Doing another encryption operation will overwrite both these files."
        );

        return;
    } else {
        println!("Decrypting...");

        let decrypted_string = super::decrypt::decrypt(config.base_bytes, config.key);
        println!("This is the decrypted string: {:?}", decrypted_string);
        return;
    }
}
