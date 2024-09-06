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
        println!("Key is being read from myrsamsg.txt...");
        let mut key_file = File::open("myrsamsg.txt")?;
        let mut key: Vec<u8> = Vec::new();
        match key_file.read_to_end(&mut key) {
            Ok(_) => {}
            Err(_) => {
                println!(
                    "myrsamsg.txt does not exist. Are you sure you encrypted your files using RSA?"
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
        let mut text_clone = config.message.clone();
        match super::encrypt::encrypt(&mut text_clone) {
            Ok(_) => {}
            Err(e) => {
                println!("{}", e);
                process::exit(90);
            }
        }

        println!(
            "The public key for this encryption has been saved in rsapublic.txt.
            The private key for this encryption has been saved in rsaprivate.txt.
            The encrypted message has been saved in my_rsa_msg.txt.
            Doing another encryption operation will overwrite all of these files."
        );

        return;
    } else {
        println!("Decrypting...");
        let mut msg_clone = config.message.clone();

        match super::decrypt::decrypt(&mut msg_clone) {
            Ok(_) => {}
            Err(e) => {
                println!("Received an error while trying to decrypt: {}", e);
                process::exit(85);
            }
        }
        println!(
            "This is the decrypted string: {:?}",
            String::from_utf8(msg_clone)
        );

        return;
    }
}
