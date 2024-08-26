use num::BigUint;
use num_primes::Generator;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
/*


RSA Cryptography

    Request two prime numbers from the user and verify them.
    Put the prime numbers in different variables.
    Determine n = pq.
    Calculate (n) = (p - 1)(q - 1)after the above step.
    Select a random number e that is close to being prime to both n and 1 e n.
    Determine d = e-1 mod n.
    Print out the private and public keys.
    Request a message from the user and then save it in a variable.
    Use the public key to encrypt the message.
    Using the private key, decrypt the message.
    Print the message, both encrypted and decrypted.


    p = large prime
    q = large prime
    n = p x q
    r = (p-1)(q-1)
    e = 3, 5, 17, 65537
    d = e-1 * mod(r)

    encrypt: m^e mod(n)

    decrypt: c^d mod(n)

*/

pub fn encrypt(vec: &mut Vec<u8>) -> Result<(), Box<dyn Error>> {
    let mut padding = 128 - (vec.len() % 128);
    if padding == 16 {
        padding = 0;
    }

    for _ in 0..padding {
        vec.push(padding as u8);
    }

    let num_blocks = vec.len() / 128;
    let msg_file = File::open("rsapublickey.txt")?;
    let reader = BufReader::new(msg_file);

    let line = reader.lines().next().ok_or("Public key file is empty")??;
    let mut nums = line.split_whitespace();

    let n = nums.next().ok_or("Missing first number")?.parse::<u64>()?;
    let e = nums.next().ok_or("Missing second number")?.parse::<u64>()?;

    if nums.next().is_some() {
        return Err("Too many numbers in this line".into());
    }

    for i in 0..num_blocks {
        let start = 128 * i;
        encrypt_block(&mut vec, n, e, start);
    }

    let mut result_file = File::create("myrsamsg.txt")?;
    result_file.write_all(&vec);

    return Ok(());
}

fn encrypt_block(vec: &mut Vec<u8>, n: u64, e: u64, start: usize) {
    for i in start..start + 128 {
        let elem = &mut vec[i];

        for _ in 0..n {
            let temp: u64 = (*elem as u64) * n;
            *elem = (temp % e) as u8;
        }
    }
}
