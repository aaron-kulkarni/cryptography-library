mod aes;
use aes::utils::KeyLength;
mod cli;
use dialoguer::Select;

#[derive(Debug)]
enum Algorithm {
    AES,
    SHA,
    RSA,
}

fn main() {
    let algo = init_algo_config();

    match algo {
        Algorithm::AES => {
            let config = match aes::cli::init_aes_config() {
                Ok(x) => x,
                //TODO: exit gracefully
                Err(_) => return,
            };
            aes::cli::run_aes(config);
        }
        Algorithm::SHA => {
            println!("Not implemented yet!")
        }
        Algorithm::RSA => {
            println!("Not implemented yet!")
        }
    }
}

fn init_algo_config() -> Algorithm {
    println!("Welcome. What encryption algorithm would you like to use?");

    let algo = match Select::new()
        .with_prompt("Do you want to encrypt or decrypt a message?")
        .default(0)
        .item("AES")
        .item("SHA")
        .item("RSA")
        .interact()
        .unwrap()
    {
        0 => Algorithm::AES,
        1 => Algorithm::SHA,
        2 => Algorithm::RSA,
        _ => panic!("I don't think this is possible"),
    };

    return algo;
}
