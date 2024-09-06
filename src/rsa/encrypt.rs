use num_bigint::BigUint;
use num_primes::Generator;
use num_traits::CheckedMul;
use num_traits::ToPrimitive;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufRead, BufReader};
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
    if padding == 128 {
        padding = 0;
    }

    for _ in 0..padding {
        vec.push(padding as u8);
    }

    let num_blocks = vec.len() / 128;
    let msg_file = File::open("rsapublic.txt")?;
    let reader = BufReader::new(msg_file);

    let mut lines = reader.lines();
    let n = lines
        .next()
        .ok_or("Error with first line")??
        .parse::<BigUint>()?;

    let e = lines
        .next()
        .ok_or("Error with second line")??
        .parse::<usize>()?;

    let mut result: Vec<u8> = Vec::new();

    for i in 0..num_blocks {
        let start = 128 * i;
        result.append(&mut encrypt_block(vec, &n, e, start));
    }

    let mut result_file = File::create("myrsamsg.txt")?;
    result_file.write_all(&result)?;

    return Ok(());
}

fn encrypt_block(vec: &mut Vec<u8>, n: &BigUint, e: usize, start: usize) -> Vec<u8> {
    let e_big = BigUint::from(e);
    let v_big = BigUint::from_bytes_be(&vec[start..start + 128]);
    return v_big.modpow(&e_big, n).to_bytes_be();
}

// for i in start..start + 128 {
//     let elem = &mut vec[i];

//     let n_usize = match nclone.to_usize() {
//         Some(value) => value,
//         None => {
//             println!("Couldn't convert BigUint to usize.");
//             return;
//         }
//     };

//     let nbiguint = BigUint::from(n);

//     for _ in 0..n_usize {
//         let temp = BigUint::from(*elem) * nbiguint;
//         // Convert `temp % e` to a primitive integer before casting to `u8`
//         let remainder = (temp % BigUint::from(e)).to_u8().unwrap_or(0);
//         *elem = remainder;
//     }
// }
//

// for i in start..start + 128 {
//     let elem = &mut vec[i];
//     for _ in 0..e {
//         let temp = BigUint::from(*elem) * n;
//         let remainder = temp
//             .modpow(&BigUint::from(1u32), &BigUint::from(e))
//             .to_u8()
//             .unwrap_or(0);
//         *elem = remainder;
//     }
// }
//
