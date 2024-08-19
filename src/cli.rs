use super::AESConfig;
use super::KeyLength;
use dialoguer::{Input, InputValidator, Select};
use std::error::Error;

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
        let base_string = Input::new()
            .with_prompt("What string do you want to encrypt?")
            //.validate_with()
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
        let mut key = String::new();
        if use_own_key == true {
            key = Input::new()
                .with_prompt(format!("Enter your {} byte key.", key_length))
                //.validate_with()
                .interact_text()?;
        } //else {
          // key = key_generator(key_length);
          // }

        return Ok(AESConfig {
            base_string,
            key_length,
            encrypt,
            key,
        });
    } else {
        //decryption
        let base_string: String = Input::new()
            .with_prompt("Provide your encrypted string.")
            //.validate_with()
            .interact_text()?;
        let key: String = Input::new()
            .with_prompt("Provide the key you used during encryption.")
            //.validate_with()
            .interact_text()?;
        let key_length = match key.len() {
            16 => KeyLength::Len16,
            24 => KeyLength::Len24,
            32 => KeyLength::Len32,
            _ => panic!("this should not happen"),
        };
        return Ok(AESConfig {
            base_string,
            key_length,
            encrypt,
            key,
        });
    }
}
