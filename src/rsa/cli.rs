use super::utils::RSAConfig;
use dialoguer::{Input, Select};
use std::error::Error;
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
            .validate_with(|s: &String| -> Result<(), String> {
                if s.len() > 128 {
                    let msg = format!(
                        "Your string was {} bytes. \
                        Please limit string length to 128 bytes.",
                        s.len()
                    );
                    Err(msg)
                } else {
                    Ok(())
                }
            })
            .interact_text()
            .unwrap();
        return Ok(RSAConfig {
            encrypt,
            generate,
            message: message.into_bytes(),
        });
    } else {
        //decryption
        //dummy value for message, will read actual value later in decrypt function.
        let message: Vec<u8> = Vec::new();

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
                    process::exit(1);
                }
            }
        }
        println!("Encrypting...");
        let mut text_clone = config.message.clone();
        match super::encrypt::encrypt(&mut text_clone) {
            Ok(_) => {}
            Err(e) => {
                println!("{}", e);
                process::exit(1);
            }
        }

        println!(
            "The public key for this encryption has been saved in rsapublic.txt.
            The private key for this encryption has been saved in rsaprivate.txt.
            The encrypted message has been saved in rsamsg.txt.
            Doing another encryption operation will overwrite all of these files."
        );

        return;
    } else {
        println!("Decrypting...");

        match super::decrypt::decrypt() {
            Ok(_) => {}
            Err(e) => {
                println!("Received an error while trying to decrypt: {}", e);
                process::exit(1);
            }
        }
        return;
    }
}
