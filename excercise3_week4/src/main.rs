use std::io;

struct BankAccount {
    username: String,
    balance: u32,
    deposit: u32,
    withdraw: u32,

}

fn main() {
    let user1 = BankAccount {
        username: String::from("janedoe"),
        balance: 30000,
        withdraw: 1000,
        deposit: 2000,
    };
    
    println!("Welcome {}", user1.username);
    println!("Enter 1 to check balance, 2 to deposit, or 3 to withdraw: ");

    let mut input_string = String::new();

    io::stdin().read_line(&mut input_string).expect("Failed to read line");
    let option = input_string.trim();
    if option == "1" {
        println!("Your balance is: {}", user1.balance);
    } else if option == "2" {
        println!("You've deposited ₦{}", user1.deposit);
    } else if option == "3" {
        println!("You've withdrawn ₦{}", user1.withdraw);
    } else {
        println!("Involuntary exit");
    }
}
