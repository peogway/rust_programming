use std::io;
use std::thread;
use std::time::Duration;

fn start_duel() {
    let wait = thread::spawn(|| {
        println!("FIRE!!!");
        thread::sleep(Duration::from_secs(5));
    });
    wait.join();

    thread::sleep(Duration::from_secs(3));
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    if input.trim() == "f" {
        println!("You fire first!");
        return;
    } else {
        println!("Oh no! You missed!");
    }

    println!("Opponent shoots first!");
}


fn main() {
    start_duel();
}
