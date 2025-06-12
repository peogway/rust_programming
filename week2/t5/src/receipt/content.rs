use crate::receipt::product::StoreProduct;

pub struct ReceiptContent {
    pub products: Vec<StoreProduct>,
    pub store: String,
}

pub fn add_to_cart(rc: &mut ReceiptContent, product: StoreProduct) {
    rc.products.push(product);
}

pub fn remove_from_cart(rc: &mut ReceiptContent) {
    rc.products.pop();
}

pub fn create_receipt(store: String) -> ReceiptContent {
    ReceiptContent {
        products: Vec::new(),
        store,
    }
}

pub fn  complete_purchase(rc: &mut ReceiptContent) -> Result<(), String> {
    println!("{}", rc.store);
    println!("------------------------------");
    let mut i: usize = 0;
    loop {
        let product_name = &rc.products.get(i).unwrap().name;
        let mut count : i32 = 0;
        let mut j: usize = 0;
        while j < rc.products.len() {
            if &rc.products.get(j).unwrap().name  == product_name {
                if j < i {
                    continue; // Skip already counted products
                }
                count += 1;
            }
            j += 1;
        }
        if j < i {
            i+=1;
            continue; // Skip already counted products
        }
        println!("{} ({}) - {}€", product_name, count, rc.products.get(i).unwrap().price*count);
        i+=1;
        if i >= rc.products.len() {
            break;
        }
    }
    println!("------------------------------");
    println!("Final price: {}€", rc.products.iter().map(|p| p.price).sum::<i32>());
    println!("------------------------------");
    Ok(())
}