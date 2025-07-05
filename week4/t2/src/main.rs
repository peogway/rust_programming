use std::fs;
use std::io::{self, Write};
use std::env;

fn read_file(filename: &str) {
    let contents = fs::read_to_string(filename);
    match contents{
        Ok(data) => {
            println!("The contents of the file:");
            println!();
            println!("{}", data);
        }
        Err(e) => {
            eprintln!("Error reading file {}: {}", filename, e);
        }
    }
}

fn write_in_file(filename: &str, content: &str) -> std::io::Result<()>  {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(filename)?;

    write!(file, "{}", content)?;
    Ok(())
}

fn main() {
    let args : Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("No arguments were given.");
        return;
    }
    if args[2] != "read" && args.len() < 4 {
        println!("More arguments needed if not reading.");
        return;
    }

    if args.len() == 3 && args[2] == "read" {
        read_file(&args[1]);
        return;
    }
    
    if args.len() == 4 && args[2] == "write" {
        let filename = &args[1];
        let content = &args[3];
        write_in_file(filename, content);
        return;
    }
    
    if args[2] == "write" && args.len() != 4 {
        
        println!("Only give three arguments when writing.");
        return;
    }
    // println!("Only give three arguments when writing.");



}
