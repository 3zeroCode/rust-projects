use std::fs;
use std::io;
fn main(){

let mut source: String = String::new();
let mut destination: String = String::new();


println!("Enter the source file here! Source:");
io::stdin().read_line(&mut source).expect("invalid file");
source = source.trim().to_string();

println!("Enter the destination file here! Destination:");
io::stdin().read_line(&mut destination).expect("try again");
destination = destination.trim().to_string();


match fs::copy(&source, &destination){
    Ok(bytes) => println!("successfully Copied {} bytes", bytes),
    Err(e) => println!("error copying file: {}", e),
}

}