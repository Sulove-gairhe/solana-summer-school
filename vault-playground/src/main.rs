struct BankVault {
    name: String,
    balance: u64,
}

impl BankVault {
    fn get_balance(&self) -> u64 {
        self.balance
    }

    fn deposit(&mut self, amount: u64) {
        self.balance += amount
    }

    fn withdraw(&mut self, amount: u64) {
        self.balance -= amount
    }
}

fn main() {
    let mut vault = BankVault {
        name: String::from("John Doe"),
        balance: 1000,
    };

    println!("{}'s balance before: {}", vault.name, vault.get_balance());

    vault.deposit(1500);

    println!("{}'s balance after: {}", vault.name, vault.get_balance());

    vault.withdraw(1000);
    println!("Balance after withdrawing 1000: {}", vault.get_balance())
}
