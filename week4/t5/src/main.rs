use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use std::env;

fn start() {
    let args: Vec<String> = env::args().collect();

    let ch = args[1].clone();;
    let time = args[2].parse::<u64>().unwrap_or(1);
    let time_clone = time;

    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    let tx2 = tx.clone();
    let ch_clone = ch.clone();
    let mut count = 0;

    thread::spawn(move || {
        thread::sleep(Duration::from_secs(time_clone));
        tx1.send(0).unwrap();
    });

    thread::spawn(move || {
        let mut input = String::new();
        loop {
            input.clear();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            let input = input.trim();
            if input == ch_clone {
                tx2.send(1).unwrap();
            }
        }
    });

    for received in rx {
        if received == 0 {
            println!("You have managed to press '{}' {} times.", ch, count);
            std::process::exit(0);
        } else if received == 1 {
            count += 1;
            println!("Presses: {}", count);
        }
    }

}

fn main() {
    println!("Do you want to start or exit?");
    print!("$ ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input = input.trim().to_lowercase();
    if input == "exit" {
        println!("Goodbye then.");
        return;
    }

    start();
}