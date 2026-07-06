fn step_one(balance: &mut u64) -> Result<(), String> {
    if *balance < 50 {
        return Err(String::from("Balance too low for step 1"));
    }

    *balance -= 50;
    return Ok(());
}

fn step_two(balance: &mut u64) -> Result<(), String> {
    if *balance < 50 {
        return Err(String::from("Balance not enough for step 2."));
    }

    *balance -= 50;
    return Ok(());
}

fn execute_both(balance: &mut u64) -> Result<(), String> {
    step_one(balance)?;
    step_two(balance)?;
    Ok(())
}

fn main() {
    let mut balance: u64 = 80; //80 is low 

    match execute_both(&mut balance) {
        Ok(()) => println!("All step is done succesfully."),
        Err(e) => println!("Failed: {}", e),
    }
}
