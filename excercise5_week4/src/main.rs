use std::io;

#[derive(Debug)]
struct Student {
    name: String,
    score: u32,
}

impl Student {
    fn passed(&self) -> bool {
        self.score >= 40
    }

    fn new_student(name: String, score: u32) -> Student {
        Student { name, score}
    }
}

fn main() {
    let mut name = String::new();
    let mut input_score = String::new();

    println!("Enter Student Name: ");
    io::stdin().read_line(&mut name).expect("Failed to read line");

    println!("Enter Student Score: ");
    io::stdin().read_line(&mut input_score).expect("Failed to read line");

    let score: u32 = input_score.trim().parse().expect("Enter valid score");

    let student = Student::new_student(name.trim().to_string(), score);

    println!("Student: {:?}", student);

    if student.passed() {
        println!("Passed the course!");
    } else {
        println!("Failed the course!");
    }
}
