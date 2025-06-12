use std::io;

fn add(x: i32, y: i32) -> i32 {
    x + y
}
fn subtract(x: i32, y: i32) -> i32 {
    x - y
}
fn multiply(x: i32, y: i32) -> i32 {
    x * y
}
fn divide(x: i32, y: i32) -> i32 {
    x/y
}

fn main() {
    let mut input = String::new();
    let mut current : i32 = 0;
    loop {
        println!("| 1) Reset | 2) Addition | 3) Retraction | 4) Multiplication | 5) Division | 6) Print | 0) End program |");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read Line");
        let choice: i32 = match input.trim().parse() {
            Ok(num) => num, 
            Err(_) => { 
                println!("Invalid input, please enter a number.");
                input.clear();
                continue;
            }
        };
        match choice {
            1 => {
                current = 0;
            }
            2 => {
                println!("What number?");
                let mut number = String::new();
                io::stdin()
                    .read_line(&mut number)
                    .expect("Failed to read Line");
                let number: i32 = match number.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid input, please enter a number.");
                        continue;
                    }
                };
                current = add(current, number);
            }
            3 => {
                println!("What number?");
                let mut number = String::new();
                io::stdin()
                    .read_line(&mut number)
                    .expect("Failed to read Line");
                let number: i32 = match number.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid input, please enter a number.");
                        continue;
                    }
                };
                current = subtract(current, number);
            }
            4 => {
                println!("What number?");
                let mut number = String::new();
                io::stdin()
                    .read_line(&mut number)
                    .expect("Failed to read Line");
                let number: i32 = match number.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid input, please enter a number.");
                        continue;
                    }
                };
                current = multiply(current, number);
            }
            5 => {
                println!("What number?");
                let mut number = String::new();
                io::stdin()
                    .read_line(&mut number)
                    .expect("Failed to read Line");
                let number: i32 = match number.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid input, please enter a number.");
                        continue;
                    }
                };
                current = divide(current, number);
            }
            6 => {
                println!("Current number: {}", current);
            }
            0 => {
                println!("Ending the program.");
                break;
            }
            _ => {
                println!("Invalid choice, please try again.");
            }
        }
        input.clear(); // Clear the input buffer for the next iteration
    }
}
