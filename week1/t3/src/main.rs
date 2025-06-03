use std::io;

fn main() {
    let mut sum :i32 = 0; 
    
    println!("By how much do you want to increment the number?");
    loop {

        println!("Current: {}. Increment by: ", sum);

        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");
        let number: i16 = match user_input.trim().parse() {
            Ok(num) => num, 
            Err(_) =>{
                println!("Parsing failed. Was the number too long for a 16-bit variable?");
                continue;
            }
        };


        
        if number == 0 {
             println!("The given value is 0. Ending the program.");
            break;
        }
        if number < 0 {
            println!("The given value is lower than 0.");
            continue;
        }
        sum = sum + number as i32;
        if sum > 32767 {
            println!("Enough incrementations.");
            break;
        }

    }
    
}
