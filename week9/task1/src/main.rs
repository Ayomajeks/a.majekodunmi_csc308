use std::thread;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    for i in 1..=3 {
        let thread_tx = tx.clone();
    }
    thread::spawn(move || {
        for i in 1..6 {
            println!("worker: {}", i);
        }
    }) 
}
