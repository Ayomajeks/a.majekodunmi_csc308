use std::io;

fn main() {
    println!("Enter temperature to convert:");

    let mut input_string = String::new();

    io::stdin().read_line(&mut input_string).expect("Failed to read line");
    let temperature_value: f64 = input_string.trim().parse().expect("Please enter a valid floating-point number!");

    println!("Is temperature celsius or fahrenheit:");
    let mut units_string = String::new();

    io::stdin().read_line(&mut units_string).expect("Failed to read line");
    let units = units_string.trim().to_lowercase();

    if units == "celsius" {
        let fahrenheit = (temperature_value * 1.8) + 32.0;
        println!("{}°C = {}°F", temperature_value, fahrenheit);
    } else if units == "fahrenheit" {
        let celsius = (temperature_value - 32.0) * 5.0/9.0;
        println!("{}°F = {}°C", temperature_value, celsius);
    } else {
        println!("Wrong Unit, Enter celsius or fahrenheit");
    }
}
