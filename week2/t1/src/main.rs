use std::io;


fn main() {
    fn create_default() -> String {
        "I want to be changed.".to_string()
    }    
    let mut s = create_default();
    loop {
        let mut input  = String::new();
        println!("| 1) Reset | 2) Remove a word | 3) Add a word | 4) Print string | 0) End program |");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input: u8 = match input.trim().parse() {
            Ok(num) =>  num,
            Err(_) => {
                println!("Parsing error. Was the input a number?");
                continue;
            }
        };
        match input {
            1 => {
                s = create_default();
            },
            2 => {
                while s.len() > 0 && !s.ends_with(' ') {
                    s.pop();
                }
                if s.len() > 0 {
                    s.pop();
                }
            },
            3 => {
                println!("The new word:");
                let mut word = String::new();
                io::stdin().read_line(&mut word).expect("Failed to read line");
                let word = word.trim();
                s += &format!(" {}", word);
            },
            4 => {
                println!("{}", s);
            },
            0 => {
                break;
            },
            _ => println!("Invalid option, please try again."),
        }
    }
}
