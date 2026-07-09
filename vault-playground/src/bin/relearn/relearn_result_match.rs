fn withdraw(balance: &mut u64, amount: u64) -> Result<(), String> {
    if *balance < amount {
        return Err(format!(
            "Cannot withdraw {amount}; only {balance} is available"
        ));
    }

    *balance -= amount;
    Ok(())
}

fn deposit(balance: &mut u64, amount: u64) -> Result<(), String> {
    if (*balance + amount) < *balance {
        return Err(format!("Cannot deposit {amount}; balance would overflow"));
    }

    *balance += amount;
    Ok(())
}

fn main() {
    let mut balance = 700;

    match withdraw(&mut balance, 200) {
        Ok(()) => println!("First withdrawal worked. Balance: {balance}"),
        Err(error) => println!("First withdrawal failed: {error}"),
    }

    match withdraw(&mut balance, 900) {
        Ok(()) => println!("Second withdrawal worked. Balance: {balance}"),
        Err(error) => println!("Second withdrawal failed: {error}"),
    }
}
