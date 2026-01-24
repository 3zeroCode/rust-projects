use std::io::{self,Write};
use sha2::{Sha256, Digest};

fn main(){

    println!("--- Sha256 Encrypter Cli---");
    println!("Enter the string you want to encrypt");
    io::stdout().flush().expect("Error try again");

    let mut user_input:String =String::new();
    io::stdin().read_line(&mut user_input).expect("error reading the string");

    let trim_input = user_input.trim();
    let mut string_hasher = Sha256::new();
    string_hasher.update(user_input.as_bytes());
    let result = string_hasher.finalize();

    println!("Orginal String: {} ", trim_input);
    println!("Hashed String output: {:?}", result);


}