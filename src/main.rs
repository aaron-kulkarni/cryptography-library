use std::{env, process};

mod aes;

fn main() {
    let config = aes::utils::Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let string_bytes: [u8; 16] = conv_string_to_bytes(&config.base_string);
    aes::encrypt::encrypt(config.key_length, string_bytes);
}

fn conv_string_to_bytes(given: &str) -> [u8; 16] {
    let bytes = given.as_bytes();
    let string_bytes: [u8; 16] = bytes.try_into().expect("something");
    return string_bytes;
}
