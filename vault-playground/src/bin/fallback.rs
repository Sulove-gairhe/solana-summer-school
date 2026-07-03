fn withdraw(balance: &mut u64, amount: u64) -> Result<(), String> {
    if *balance < amount {
        return Err(String::from("Not enough balance"));
    }

    *balance -= amount;

    return Ok(());
}

fn main() {
    let mut balance: u64 = 1000;

    match withdraw(&mut balance, 300) {
        Ok(()) => println!("Withdraw 300 succesfull: {}", balance),
        Err(e) => println!("Error: {}", e),
    }

    match withdraw(&mut balance, 2000) {
        Ok(()) => println!("Withdraw 2000 succesfull: {}", balance),
        Err(e) => println!("Err: {}", e),
    }
}
