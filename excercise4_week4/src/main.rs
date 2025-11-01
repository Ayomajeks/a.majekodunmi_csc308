use std::io;

fn main() {
    println!("Enter Sentence:");

    let mut input_string = String::new();

    io::stdin().read_line(&mut input_string).expect("Failed to read line");
    
    let words: Vec<&str> = input_string.trim().split_whitespace().collect();

    if words.is_empty() {
        println!("No words were entered.");
        return;
    }

    let mut shortest = words[0];
    let mut longest = words[0];

    for &word in &words {
        if word.len() < shortest.len() {
            shortest = word;
        } else if word.len() > longest.len() {
            longest = word;
        }
    }

    println!("Shortest word: {}", shortest);
    println!("Longest word: {}", longest)
    
    
    
}
