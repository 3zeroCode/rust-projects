use bip39::{Mnemonic, Language, MnemonicType};
use rand::{RngCore, rngs::OsRng};

fn main() {
    println!("--- Secure Mnemonic Generator ---");

    // 1. Generate Entropy (The "Chaos")
    // We use OsRng, which pulls randomness from the operating system's 
    // entropy source (like hardware noise), making it secure for crypto.
    let mut entropy = [0u8; 32]; // 32 bytes = 256 bits
    OsRng.fill_bytes(&mut entropy);

    // 2. Create the Mnemonic
    // We map that entropy to the English word list.
    // MnemonicType::Words24 ensures we get a 24-word phrase.
    let mnemonic = Mnemonic::from_entropy(&entropy, Language::English)
        .expect("Failed to create mnemonic from entropy");

    // 3. Display the Phrase
    let phrase = mnemonic.phrase();
    
    println!("\n[CAUTION] Write these words down in order. Never share them!");
    println!("------------------------------------------------------------");
    println!("{}", phrase);
    println!("------------------------------------------------------------");

    // 4. Converting to Seed
    // This seed is what you would actually use to derive your Private Keys.
    let seed = mnemonic.to_seed(""); // Empty string is an optional password (passphrase)
    println!("\nMaster Seed (Hex): {:x?}", &seed[0..32]); 
    println!("(Showing first 32 bytes of the 64-byte seed)");
}