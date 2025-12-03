use std::process::Command;


fn main() {
    let child = Command::new("sleep") 
        .arg("5") 
        .spawn() 
        .expect("Failed to spawn");

    println!("Sleep 5 spawned with PID: {}", child.id());

    let output = Command::new("ls") 
        .arg("-la") 
        .output() 
        .expect("Failed to spawn");

    println!("Output is: {}", String::from_utf8_lossy(&output.stdout));

    let child2 = Command::new("echo")
        .arg("Hello from child") 
        .output() 
        .unwrap();

    println!("Output is: {}", String::from_utf8_lossy(&child2.stdout));

}
