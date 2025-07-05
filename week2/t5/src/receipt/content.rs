use crate::receipt::product::{StoreProduct, create_products};


use std::io;
use std::fs::File;
use std::io::Write;
use std::collections::HashMap;


pub struct ReceiptContent {
    pub products: Vec<StoreProduct>,
    pub store: String,
}


pub fn complete_purchase(rc: &mut ReceiptContent) -> Result<(), String> {
    let mut file = File::create("receipt.txt").expect("Error creating the file!");

    writeln!(file, "{}", rc.store).expect("Error writing to the file!");
    writeln!(file, "------------------------------").expect("Error writing to the file!");

    let mut counts = HashMap::new();
    let mut total = 0;

    for p in &rc.products {
        *counts.entry(p.name.clone()).or_insert((0, p.price)) = (
            counts.get(&p.name).map_or(1, |(c, _)| c + 1),
            p.price,
        );
        total += p.price;
    }

    for (name, (qty, price)) in counts {
        writeln!(file, "{} ({}) - {}€", name, qty, qty * price).expect("Error writing to the file!");
    }

    writeln!(file, "------------------------------").expect("Error writing to the file!");
    writeln!(file, "Final price: {}€", total).expect("Error writing to the file!");
    writeln!(file, "------------------------------").expect("Error writing to the file!");

    Ok(())
}

pub fn run_receipt() {
    let products = create_products();
    let mut receipt = ReceiptContent {
        products: vec![],
        store: "Imaginary Town General Store".to_string(),
    };

    loop {
        println!("| 1) Add to cart | 2) Remove most recent product | 3) Purchase |");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        match choice {
            "1" => {
                println!("Which product would you like to add?");
                for (i, p) in products.iter().enumerate() {
                    println!("{}) {} | Price - {}", i + 1, p.name, p.price);
                }
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();

                match input.trim().parse::<usize>() {
                    Ok(n) if n >= 1 && n <= products.len() => {
                        receipt.products.push(products[n - 1].clone());
                    }
                    _ => println!("Invalid input."),
                }
            }
            "2" => {
                receipt.products.pop();
            }
            "3" => {
                let _ = complete_purchase(&mut receipt);
                println!("Thank you for your purchase!\n");
                break;
            }
            _ => println!("Invalid choice."),
        }
    }
}
