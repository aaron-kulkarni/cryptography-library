use std::{env, process};

mod aes;

fn main() {
    let config = aes::utils::Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let bytes = config.base_string.as_bytes();

    if bytes.len() == 16 {
        let string_bytes: [u8; 16] = bytes.try_into().expect("something");

        aes::encrypt::encrypt(config.key_length, string_bytes);
    } else {
        eprintln!("Length of string in bytes must be 16.");
        process::exit(2);
    }
}
