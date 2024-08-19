use std::process;
mod aes;
use aes::utils::AESConfig;
use aes::utils::KeyLength;
mod cli;

fn main() {
    let config = match cli::init_aes_config() {
        Ok(x) => x,
        //TODO: exit gracefully
        Err(e) => process::exit(8),
    };

    if config.encrypt {
        println!(
            "This is the key you are using to encrypt. Make sure to save it \
            so you can later decrypt this message."
        );
        println!("{}", config.key);

        println!("Encrypting...");
        let encrypted_bytes = aes::encrypt::encrypt(&config.base_string, &config.key);
        println!(
            "This is the encrypted bytes information: {:?}",
            encrypted_bytes
        );
        return;
    } else {
        // println!("Decrypting...");
        // let decrypted_string = aes::decrypt::decrypt(&config.base_string, &config.key);
        // println!("This is the decrypted string: {:?}", decrypted_string);
        return;
    }
}
