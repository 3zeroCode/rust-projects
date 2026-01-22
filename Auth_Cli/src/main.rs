
use std::io;

struct User {
    name: String,
    email: String,
    password: String, // Changed from i8 to String
}

fn main() {
    println!("Welcome to the Auth cli system");

    let mut db: Vec<User> = Vec::new();

    loop {
        let mut choice = String::new();
        println!("\nChoose an option: (l) login, (r) register, (q) quit");
        
        io::stdin().read_line(&mut choice).expect("error");
        let user_choice = choice.trim().to_lowercase();

        match user_choice.as_str() {
            "l" => login(&db),          // Pass as read-only reference
            "r" => register(&mut db),   // Pass as mutable reference
            "q" => {
                println!("Thank you!");
                break;
            }
            _ => println!("Invalid option, try again"),
        }
    }
}

fn login(db: &Vec<User>) {
    let mut user_email = String::new();
    let mut user_password = String::new();

    println!("Enter your email:");
    io::stdin().read_line(&mut user_email).expect("Error");

    println!("Enter your password:");
    io::stdin().read_line(&mut user_password).expect("Error");

    let email = user_email.trim().to_lowercase();
    let password = user_password.trim();

    // Searching the vector
    let found_user = db.iter().find(|user| user.email == email);

    match found_user {
        Some(user) => {
            if user.password == password {
                println!("Welcome back, {}!", user.name);
            } else {
                println!("Incorrect password.");
            }
        }
        None => println!("No user found with that email."),
    }
}

fn register(db: &mut Vec<User>) {
    let mut name = String::new();
    let mut email = String::new();
    let mut password = String::new();

    println!("Enter your name:");
    io::stdin().read_line(&mut name).expect("Error");

    println!("Enter your email:");
    io::stdin().read_line(&mut email).expect("Error");

    println!("Enter your password:");
    io::stdin().read_line(&mut password).expect("Error");

    let user_info = User {
        name: name.trim().to_string(),
        email: email.trim().to_string().to_lowercase(),
        password: password.trim().to_string(), 
    };

    db.push(user_info);
    println!("User registered successfully!");
}