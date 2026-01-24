use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main(){
    println!("Welcome to the Number guessing game!");
    let secrect = rand::thread_rng().gen_range(1..100);

    loop{
        println!("Please guess a number! (1-100)");
        let mut guess = String::new();
        io::stdin()
        .read_line(&mut guess).expect("Error reading the number");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number please try again");
                continue;
            }
        };

        println!("The guess is {}", guess);

        match guess.cmp(&secrect){
            Ordering::Less => println!("The number is less"),
            Ordering::Greater => println!("The number is great "),
            Ordering::Equal => {
                println!("You win the jackpot!! Your number was {} ", secrect);
                break;
            } 
        }

    
    }

  
}