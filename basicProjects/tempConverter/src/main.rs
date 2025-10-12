use std::io;

fn main() {
    println!("Hello, world!");
    println!("Enter a temperature calculator");
    println!("1. Celsius to Fahrenheit");
    println!("2. Fahrenheit to Celsius");
    println!("Enter a  choice between 1 and 2: ");

    let mut choice = String::new();
    io::stdin()
    .read_line(&mut choice)
    .expect("Failed to read line");

    let choice: u32 = match choice.trim().parse() {
        Ok(number) => number,
        Err(_) => {
            println!("Please type a number!");
            return;
        }
    };

    if choice == 1 {
       celsius_to_fahrenheit();
    }
    else if choice == 2 {
        fahrenheit_to_celsius();
    }
    else {
        println!("Invalid choice");
    }

}

fn celsius_to_fahrenheit() {
    println!("Enter a temperature in Celsius: ");
    let mut celsius = String::new();
    io::stdin()
    .read_line(&mut celsius)
    .expect("Failed to read line");
    let celsius: f64 = celsius.trim().parse()
    .expect("Please type a number!");
    let fahrenheit = (celsius * 9.0/5.0) + 32.0;
    println!("{} Celsius is {} Fahrenheit", celsius, fahrenheit);
}   

fn fahrenheit_to_celsius() {
    println!("Enter a temperature in Fahrenheit: ");
    let mut fahrenheit = String::new();
    io::stdin()
    .read_line(&mut fahrenheit)
    .expect("Failed to read line");
    let fahrenheit: f64 = fahrenheit.trim().parse()
    .expect("Please type a number!");
    let celsius = (fahrenheit - 32.0) * 5.0/9.0; // formuala
    println!("{} Fahrenheit is {} Celsius", fahrenheit, celsius);
}
