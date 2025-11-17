use rand::Rng; 
fn ge_pswd(length: usize) -> String {
    let charset: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"; 
    let mut password: String = String::new();
    let mut rng = rand::thread_rng();

    for _ in 0..length {
        let random_index = rng.gen_range(0..charset.len()); 
        
        if let Some(c) = charset.chars().nth(random_index) {
            password.push(c);
        }
    }
    
   
    password
}

fn main() {
    const PASSWORD_LENGTH: usize = 16; 
    let password = ge_pswd(PASSWORD_LENGTH); 
    
    println!("🔑The Generated Password is of length {} and the Password is {}", PASSWORD_LENGTH, password);
}