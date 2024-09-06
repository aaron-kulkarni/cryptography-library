use num_bigint::BigUint;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufRead, BufReader};

pub fn decrypt(vec: &mut Vec<u8>) -> Result<(), Box<dyn Error>> {
    if vec.len() % 128 != 0 {
        panic!("Something went wrong with encryption. The encrypted message length should be a multiple of 128.");
    }
    let num_blocks = vec.len() / 128;

    let msg_file = File::open("rsaprivate.txt")?;
    let reader = BufReader::new(msg_file);

    let mut lines = reader.lines();

    let n = lines
        .next()
        .ok_or("Error with first line")??
        .parse::<BigUint>()?;
    let d = lines
        .next()
        .ok_or("Missing second number")??
        .parse::<usize>()?;

    let mut result: Vec<u8> = Vec::new();

    for i in 0..num_blocks {
        let start = 128 * i;
        result.append(&mut decrypt_block(vec, &n, d, start));
    }

    let mut result_file = File::create("myrsamsg.txt")?;
    result_file.write_all(&result)?;

    return Ok(());
}

fn decrypt_block(vec: &mut Vec<u8>, n: &BigUint, d: usize, start: usize) -> Vec<u8> {
    let d_big = BigUint::from(d);
    let v_big = BigUint::from_bytes_be(&vec[start..start + 128]);
    return v_big.modpow(&d_big, n).to_bytes_be();
}
