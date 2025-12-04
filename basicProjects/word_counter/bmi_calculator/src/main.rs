use std::io;

fn main(){
    println!("welcome to the BMI calculator");

    let mut weight: String = String::new();
    let mut height: String = String::new();


    println!("Enter your weight in kilogram (kg)");
    io::stdin().read_line(&mut weight).expect("error");
    let weight: f64 = match weight.trim().parse() {
        Ok(value) => value,
        Err(_) => {
            println!("error try again");
            return;
        }
    };

    println!("Enter your height in Centimeters (cm) ex (170, 180, 165)");
    io::stdin().read_line(&mut height).expect("error");
    let height: f64 = match height.trim().parse() {
        Ok(value) => value,
        Err(_) => {
            println!("Error enter your height again");
            return;
        }
    };

    if height <= 0.0 {
        println!("Height cant be lower than 0");
        return;
    }

    let height_m = height / 100.0;
    let user_bmi = calculate_bmi(weight, height_m);
    println!("Your BMI is {:.2}", user_bmi);
}


fn calculate_bmi(weight: f64, height: f64) -> f64 {
    let bmi = weight  / (height * height);
    bmi
}