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
}

fn main() {
    let mut vault = BankVault {
        name: String::from("John Doe"),
        balance: 1000,
    };

    println!("Balance Before: {}", vault.get_balance());

    vault.deposit(1500);

    println!("Balance After: {}", vault.get_balance());
}
