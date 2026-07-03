struct BankVault {
    owner: String,
    balance: u64,
}

impl BankVault {
    fn deposit(&mut self, amount: u64) {
        self.balance = self.balance + amount;
    }
}

fn main() {
    // BUG: this vault is not declared as mutable
    let mut vault = BankVault {
        //prev it was just `let vault:BankVault = BankVault{} thus error would occur`
        owner: String::from("Alice"),
        balance: 500,
    };

    vault.deposit(100); // ERROR: can't mutate an immutable variable
    println!("Balance: {}", vault.balance);
}
