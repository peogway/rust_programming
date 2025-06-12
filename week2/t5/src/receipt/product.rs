#[derive(Clone)]
#[derive(PartialEq)]
pub struct StoreProduct {
    pub name: String,
    pub price: i32,
}

pub fn create_products() -> Vec<StoreProduct> {
    vec![
        StoreProduct { name: "Zbox 720".to_string(), price: 600 },
        StoreProduct { name: "GPU - AND Random RT6600".to_string(), price: 200 },
        StoreProduct { name: "Potato".to_string(), price: 1 },
    ]
}
