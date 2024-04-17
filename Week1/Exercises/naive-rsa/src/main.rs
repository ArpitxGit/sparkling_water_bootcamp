use std::env;

mod encrypt;
mod decrypt;
mod generate;
mod utils;

use encrypt::encrypt;
use generate::generate;
use decrypt::decrypt;

fn main() {
    let default_e: u64 = 65537;

    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("Please specify a mode: encrypt, decrypt, or generate.");
        return;
    }
    
    let mode = &args[1];

    match mode.as_str() {
        "encrypt" => {
            if args.len() < 4 {
                println!("Usage: encrypt <message> <public_key>");
                return;
            }
            let m = args.get(2).unwrap();
            let public_key = args.get(3).unwrap();
            encrypt(m, public_key);
        }
        "decrypt" => {
            if args.len() < 4 {
                println!("Usage: decrypt <ciphertext> <private_key>");
                return;
            }
            let c = args.get(2).unwrap();
            let private_key = args.get(3).unwrap();
            decrypt(private_key, c);
        }
        "generate" => {
            generate(default_e, 32); // You can change the number of bytes of the RSA here
        }
        _ => {
            println!("Invalid mode. Use 'encrypt', 'decrypt', or 'generate'.");
        }
    }
}
