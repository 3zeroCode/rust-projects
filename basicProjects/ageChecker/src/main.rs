use std::io;

fn main(){

    const LEGAL_AGE: u8 = 18;

    println!("Hello, welcome to the checker");

    println!("Enter your age!");
    let mut user_age = String::new();

    io::stdin()
    .read_line(&mut user_age)
    .expect("failed to read the age");

    let age: u8 = match user_age.trim().parse(){
        Ok(num) => num,
        Err(_) => {
            println!("Invalid age please try again!");
            return;
        }
    };
    
        if age == LEGAL_AGE{
            println!("your a adult you can drink");
        } else{
            println!("sorry you cannot drink");
        }

        println!("thank you for using the checker");
    

}