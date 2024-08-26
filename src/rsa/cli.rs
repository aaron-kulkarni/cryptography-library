use super::utils::RSAConfig;
use crate::KeyLength;
use dialoguer::{Input, Select};
use num::BigUint;
use num_primes::Generator;
use rand::Rng;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::process;

pub fn init_rsa_config() -> Result<RSAConfig, Box<dyn Error>> {
    println!(
        "Good choice! Let's encrypt/decrypt your messages with the Rivest-Shamir-Adleman crypto system."
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
    let mut generate = false;
    if encrypt == true {
        generate = match Select::new()
            .with_prompt("Do you want to generate a new key pair or use an old one?")
            .default(0)
            .item("Generate New")
            .item("Use Old")
            .interact()
            .unwrap()
        {
            0 => true,
            1 => false,
            _ => panic!("how would this ever happen"),
        };

        let message: String = Input::new()
            .with_prompt("What string do you want to encrypt?")
            .interact_text()
            .unwrap();
        return Ok(RSAConfig {
            encrypt,
            generate,
            message: message.into_bytes(),
        });
    } else {
        //decryption
        println!("Key is being read from myrsakey.txt...");
        let mut key_file = File::open("myrsakey.txt")?;
        let mut key: Vec<u8> = Vec::new();
        match key_file.read_to_end(&mut key) {
            Ok(_) => {}
            Err(_) => {
                println!(
                    "myrsakey.txt does not exist. Are you sure you encrypted your files using RSA?"
                )
            }
        }

        println!("Encrypted message is being read from myrsamsg.txt...");
        let mut msg_file = File::open("myrsamsg.txt")?;
        let mut message = Vec::new();

        match msg_file.read_to_end(&mut message) {
            Ok(_) => {}
            Err(_) => {
                println!(
                    "myrsamsg.txt does not exist. Are you sure you encrypted your files using RSA?"
                )
            }
        }

        return Ok(RSAConfig {
            encrypt,
            generate: false,
            message,
        });
    }
}

pub fn run_rsa(config: RSAConfig) {
    if config.encrypt {
        if config.generate {
            println!("Generating public/private keys...");
            match super::setup::rsa_setup() {
                Ok(_) => {}
                Err(e) => {
                    println!("{}", e);
                    process::exit(10);
                }
            }
        }
        println!("Encrypting...");
        let key_clone = config.key.clone();
        let mut text_clone = config.message.clone();
        let encrypted_bytes = super::encrypt::encrypt(&mut text_clone, config.key);

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

        let decrypted_string = super::decrypt::decrypt(config.message, config.key);
        println!("This is the decrypted string: {:?}", decrypted_string);
        return;
    }
}

fn setup_rsa() {}
