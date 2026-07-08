fn pay_fee(balance: &mut u64) -> Result<(), String> {
    if *balance < 25 {
        return Err(String::from("Not enough balance to pay the fee"));
    }

    *balance -= 25;
    Ok(())
}

fn reserve_deposit(balance: &mut u64) -> Result<(), String> {
    if *balance < 100 {
        return Err(String::from("Not enough balance to reserve the deposit"));
    }

    *balance -= 100;
    Ok(())
}

fn open_vault(balance: &mut u64) -> Result<(), String> {
    pay_fee(balance)?;
    reserve_deposit(balance)?;
    Ok(())
}

fn main() {
    let mut balance = 110;

    match open_vault(&mut balance) {
        Ok(()) => println!("Vault opened. Remaining balance: {balance}"),
        Err(error) => println!("Vault opening failed: {error}"),
    }
}
