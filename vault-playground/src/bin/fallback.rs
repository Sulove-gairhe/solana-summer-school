fn withdraw(balance: &mut u64, amount: u64) -> Result<(), String> {
    if *balance < amount {
        return Err(String::from("Not enough balance"));
    }

    *balance -= amount;

    Ok(())
}

fn main() {
    let mut balance: u64 = 1000;

    match withdraw(&mut balance, 300) {
        Ok(()) => println!("Withdrawal of 300 successful: {}", balance),
        Err(e) => println!("Error: {}", e),
    }

    match withdraw(&mut balance, 2000) {
        Ok(()) => println!("Withdrawal of 2000 successful: {}", balance),
        Err(e) => println!("Err: {}", e),
    }
}
