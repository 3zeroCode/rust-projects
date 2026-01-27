#[derive(Debug)]
struct Account {
    lamports: u64,
    data: Vec<u8>,
    owner: String,
    executable: bool,
}

impl Account {
    fn new(owner: &str, lamports: u64) -> Self {
        Self {
            lamports,
            data: Vec::new(),
            owner: owner.to_string(),
            executable: false,
        }
    }

    fn deposit(&mut self, amount: u64) {
        self.lamports += amount;
        println!(
            "Funds deposited: {}. New balance is {}.",
            amount, self.lamports
        );
    }

    fn withdraw(&mut self, amount: u64) {
        if amount > self.lamports {
            println!("Withdrawal failed: insufficient funds. Current balance is {}.", self.lamports);
        } else {
            self.lamports -= amount;
            println!(
                "Funds withdrawn: {}. New balance is {}.",
                amount, self.lamports
            );
        }
    }

    fn write_data(&mut self, new_data: &str) {
        self.data = new_data.as_bytes().to_vec();
        println!(
            "The new data has been added! The data is {:?}",
            String::from_utf8_lossy(&self.data)
        );
    }

    fn check_status(&self) {
        println!("Account owner is {}", self.owner);
        println!("Account balance is {}", self.lamports);
        println!(
            "Executable: {}. Data: {:?}",
            self.executable,
            String::from_utf8_lossy(&self.data)
        );
    }
}

fn main() {
    let mut test_account = Account::new("11111111111111111111111111111111", 1000);

    test_account.deposit(1500);
    test_account.withdraw(200); 
    test_account.write_data("hello");
    test_account.check_status();
}