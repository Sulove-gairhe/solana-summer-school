struct BankVault {
    balance: u64,
}

fn print_balance(vault: &BankVault) {
    //&BankVault not &mut BankVault cuz we only need to read values and not write.
    println!("Balance: {}", vault.balance);
    // vault is borrowed, not owned — we can't move it out
}

fn add_funds(vault: &mut BankVault, amount: u64) {
    //&mut BankVault cuz we need to write new values .
    vault.balance += amount; // can modify because &mut
}

fn main() {
    let mut my_vault = BankVault { balance: 500 }; //why mut here ? Cuz we need to change the value of `balance`-Line 11 . If we didn't need to change, we could have not written it, but since we are changing it, we need to write mut .

    print_balance(&my_vault); // borrow: read-only
    add_funds(&mut my_vault, 300); // borrow: read+write
    print_balance(&my_vault); // borrow again: still works, we own it
}
