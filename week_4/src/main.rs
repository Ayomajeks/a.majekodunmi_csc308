use std::io;

fn last_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate().rev() {
        if item == b' ' {
            return &s[i + 1..s.len()].trim();
        }
    }

    s.trim()
}

fn main() {
    println!("Enter String to Slice:");

    let mut input_string = String::new();

    io::stdin().read_line(&mut input_string).expect("Failed to read line");
    
    let your_string = String::from(input_string);
    let word: &str = last_word(&your_string);
    println!("The last word is: {}", word);
    
    
    
}
