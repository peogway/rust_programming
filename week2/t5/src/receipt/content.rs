use crate::receipt::product::{StoreProduct, create_products};


use std::io;
pub struct ReceiptContent {
    pub products: Vec<StoreProduct>,
    pub store: String,
}

pub fn complete_purchase(rc: &mut ReceiptContent) -> Result<(), String> {
    
    println!("Thank you for your purchase!\n");

    println!("{}", rc.store);
    println!("------------------------------");

    let mut counts = std::collections::HashMap::new();
    let mut total = 0;

    for p in &rc.products {
        *counts.entry(p.name.clone()).or_insert((0, p.price)) = (
            counts.get(&p.name).map(|(c, _)| c + 1).unwrap_or(1),
            p.price,
        );
        total += p.price;
    }

    for (name, (qty, price)) in counts {
        println!("{} ({}) - {}€", name, qty, qty * price);
    }

    println!("------------------------------");
    println!("Final price: {}€", total);
    println!("------------------------------");
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
                complete_purchase(&mut receipt);
                break;
            }
            _ => println!("Invalid choice."),
        }
    }
}
