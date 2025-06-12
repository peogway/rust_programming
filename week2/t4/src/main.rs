pub mod map;

use map::arraymap::*;
use std::io;

fn main() {
    let mut input = String::new();
    let created_map: [[char; 5]; 5] = create_map();
    let mut pos :i32 = 12;

    loop {
        print_map(pos);
        println!("| w) Move up | a) Move left | s) Move down | d) Move right | e) End program |");
        let _ = io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let choice = input.trim().to_lowercase();
        input.clear();
        match choice.as_str() {
            "w" => move_up(&mut pos),
            "a" => move_left(&mut pos),
            "s" => move_down(&mut pos),
            "d" => move_right(&mut pos),
            "e" => {
                println!("Ending the program.");
                break;
            }
            _ => println!("Invalid choice, please try again."),
        }

    }
}