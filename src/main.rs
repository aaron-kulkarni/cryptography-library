use std::{env, process};

mod aes;

fn main() {
    let config = aes::utils::Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let encrypted_string: String = aes::encrypt::encrypt(config.key_length, config.base_string);
    println!("This is the encrypted string: {}", encrypted_string);
}

// fn conv_string_to_bytes(given: &str) -> [u8; 16] {
//     let bytes = given.as_bytes();
//     let string_bytes: [u8; 16] = bytes.try_into().expect("something");
//     return string_bytes;
// }

// fn main() {
//     let a = String::from("abcdefghijklmnop");
//     let bytes = a.as_bytes().to_vec();

//     let toprint = String::from_utf8(bytes);

//     match toprint {
//         Ok(_) => println!("{:?}", toprint),
//         Err(_) => println!("terrible"),
//     }
// }
