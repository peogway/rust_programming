pub mod receipt;

use receipt::content::*;
use receipt::product::*;

use std::io;

fn main() {
    let products = create_products();

    let mut receipt = create_receipt(String::from("Imaginary Town General Store"));

    loop {
        println!("| 1) Add to cart | 2) Remove most recent product | 3) Purchase |");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let choice: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input, please enter a number.");
                continue;
            }
        };
        match choice {
            1 => {
                println!("Which product would you like to add?");
                print_products(&products);
                let mut product_choice = String::new();
                io::stdin().read_line(&mut product_choice).expect("Failed to read line");

                let product_index: usize = match product_choice.trim().parse::<usize>() {
                    Ok(num) => num - 1, // Convert to zero-based index
                    Err(_) => {
                        println!("Invalid input, please enter a valid product number.");
                        continue;
                    }
                };

                if product_index < products.len() {
                    add_to_cart(&mut receipt, products[product_index].clone());
                } else {
                    println!("Invalid product number.");
                }
            }
            2 => {
                remove_from_cart(&mut receipt);
            }
            3 => {
                println!("Thank you for your purchase!");
                // complete_purchase(&mut receipt).expect("Failed to complete purchase");
                break; // Exit the loop after purchase
            }
            _ => println!("Invalid choice, please try again."),
        }
        // complete_purchase(&mut receipt);
    }

}
