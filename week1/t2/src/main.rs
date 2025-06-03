use std::io;

fn main() {
    let _s1 = String::from("Rust");
    let _s2 = String::from("No");

    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");
    let user_input = user_input.trim();

    match user_input.to_lowercase().as_str() {
        "rust" => {
            println!("So you appreciate Rust? That's great! Thank you!");
        },
        "no" => {
            println!("So you like nothing? Alright then... :)");
        },
        _ => {
            println!("It seems that you like {}.", user_input);
        }
    }
}
