struct StudyVault {
    owner: String,
    balance: u64,
}

impl StudyVault {
    fn new(owner: String, balance: u64) -> Self {
        Self { owner, balance }
    }

    fn get_balance(&self) -> u64 {
        self.balance
    }

    fn deposit(&mut self, amount: u64) {
        self.balance += amount;
    }

    fn withdraw(&mut self, amount: u64) {
        self.balance -= amount;
    }
}

fn main() {
    let mut vault = StudyVault::new(String::from("Student"), 1_000);

    println!("{} starts with {}", vault.owner, vault.get_balance());

    vault.deposit(500);
    println!("After deposit: {}", vault.get_balance());

    vault.withdraw(250);
    println!("After withdraw: {}", vault.get_balance());
}
