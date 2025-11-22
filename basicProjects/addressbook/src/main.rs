use std::io;

fn main() {
    println!("Welcome to the address book program");

    let mut name: String = String::new();
    let mut db: Vec<String> = Vec::new();
    println!("Please Enter Your Name");
    io::stdin().read_line(&mut name).expect("Error");
    db.push(name);
    println!("Your name has been registered");

    let mut address: String = String::new();
    println!("Please Enter Your Address");
    io::stdin().read_line(&mut address).expect("error");
    db.push(address);
    println!("your Address has been registered");

    println!("Your name is {} and address is {}", db[0].trim_end(), db[1].trim_end());

}
