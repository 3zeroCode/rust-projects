use std::io;

fn main(){

println!("Hello to the Simple word counter");
println!("Enter word, sentence or pharse the you want to be counted");


let mut user_input: String = String::new();
io::stdin().read_line(&mut user_input).expect("error try again");

let word_count = user_input.trim().split_whitespace().count();
println!("The total count is {}", word_count);

}