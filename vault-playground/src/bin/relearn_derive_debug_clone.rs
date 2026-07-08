#[derive(Debug, Clone)]
struct StudyVault {
    owner: String,
    balance: u64,
}

fn main() {
    let original = StudyVault {
        owner: String::from("Alice"),
        balance: 1_000,
    };

    let mut copied = original.clone();
    copied.owner = String::from("Bob");
    copied.balance = 1_500;

    println!("Original: {:?}", original);
    println!("Copied: {:?}", copied);
}
