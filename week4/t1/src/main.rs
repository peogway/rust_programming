use std::io;


fn read_file() {
    match std::fs::read_to_string("read.txt"){
        Ok(content) => {
            println!("$ {}", content);
        }
        Err(e) => {
            println!("Error reading file: {}", e);
        }
    }
}

fn prank_user(){
    println!("$ You have received an email.");
}

fn main() {
    let mut input = String::new();
    loop {
        input.clear(); 
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let trimmed = input.trim();

        match trimmed {
            "read" => read_file(),
            "prank" => prank_user(),
            "help" => println!("$ Commands: read, prank, help, end."),
            "end" => break,
            _ => println!("$ Invalid command. Try again."),
        }
    
    }
}