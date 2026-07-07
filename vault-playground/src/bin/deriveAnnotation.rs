use std::clone;

#[derive(Debug, Clone)]
struct BankVault {
    owner: String,
    balance: u64,
}

fn main() {
    let vault = BankVault {
        owner: String::from("Alice"),
        balance: 1000,
    };

    let mut vault_clone = vault.clone();
    vault_clone.owner = String::from("Bob");
    vault_clone.balance = 2000;

    println!("{:?}", vault);
    println!("{:?}", vault_clone);
}
