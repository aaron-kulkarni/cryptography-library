use num_bigint::BigUint;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process;

pub fn decrypt() -> Result<(), Box<dyn Error>> {
    let msg_file = File::open("rsamsg.txt")?;
    let msg_reader = BufReader::new(msg_file);
    let cipher = msg_reader
        .lines()
        .next()
        .ok_or("Error with cipher")??
        .parse::<BigUint>()?;

    let key_file = File::open("rsaprivate.txt")?;
    let reader = BufReader::new(key_file);

    let mut lines = reader.lines();

    let n = lines
        .next()
        .ok_or("Error with first line")??
        .parse::<BigUint>()?;
    let d = lines
        .next()
        .ok_or("Missing second number")??
        .parse::<BigUint>()?;

    let mut result = cipher.modpow(&d, &n).to_bytes_be();

    remove_padding(&mut result);
    println!(
        "{}",
        String::from_utf8(result).unwrap_or(String::from("Error printing decrypted string"))
    );
    return Ok(());
}

fn remove_padding(text: &mut Vec<u8>) {
    /* Padding scheme example:
       User passes in a plaintext string which is 20 bytes.
       We need to add 12 bytes so that the string length is divisible by 16.
       We add 12 bytes of the byte representation of 12.
       This way, it's clear to the decryption algorithm which bytes are padding
       and which are not.
    */

    let last_byte: usize = match text.last() {
        Some(a) => *a as usize,
        None => {
            println!("Decryption key seems to be invalid.");
            process::exit(1)
        }
    };

    if text
        .iter()
        .rev()
        .take(last_byte)
        .all(|&byte| byte == last_byte as u8)
    {
        text.truncate(text.len() - last_byte);
    }
}
