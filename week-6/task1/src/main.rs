use std::io;

fn main() {
    println!("Enter value to find factorial of: ");
    let mut input_string = String::new();

    io::stdin().read_line(&mut input_string).expect("Failed to read line");
    let option: u32 = input_string.trim().parse().expect("Enter valid number!");

    let factorial = |o: u32| {
        let mut k = 1;
        for i in 1..=o {
            k *= i;
        }
        return k
    };

    println!("Factorial of {} is {}", option, factorial(option));

}
