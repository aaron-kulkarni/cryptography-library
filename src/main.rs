mod aes;
mod rsa;
use aes::utils::KeyLength;
use dialoguer::Select;

#[derive(Debug)]
enum Algorithm {
    AES,
    RSA,
}

fn main() {
    let algo = init_algo_config();

    match algo {
        Algorithm::AES => {
            let config = match aes::cli::init_aes_config() {
                Ok(x) => x,
                Err(e) => {
                    println!("{}", e);
                    return;
                }
            };
            aes::cli::run_aes(config);
        }
        Algorithm::RSA => {
            let config = match rsa::cli::init_rsa_config() {
                Ok(x) => x,
                Err(e) => {
                    println!("{}", e);
                    return;
                }
            };
            rsa::cli::run_rsa(config);
        }
    }
}

fn init_algo_config() -> Algorithm {
    println!("Welcome. What encryption algorithm would you like to use?");

    let algo = match Select::new()
        .with_prompt("Do you want to encrypt or decrypt a message?")
        .default(0)
        .item("AES")
        .item("RSA")
        .interact()
        .unwrap()
    {
        0 => Algorithm::AES,
        1 => Algorithm::RSA,
        _ => panic!("I don't think this is possible"),
    };

    return algo;
}
