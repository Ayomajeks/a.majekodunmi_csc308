use std::process::Command;

fn main() {
    let output = Command::new("echo")
        .arg("Hello from child process!") 
        .output() 
        .expect("Failed to execute process");
    println!("Child process status: {:?}", output.status);
    println!("Child Process: {}", String::from_utf8_lossy(&output.stdout));
}
