use rand::Rng;
use crate::gear::Item::{Item};

pub struct Items{ 
    items: Vec<Item>,
}

impl Items {
    pub fn new() -> Self {
        let items = vec![
            Item::new("Sword".to_string(), 10.0, 2.0),
            Item::new("Shield".to_string(), 2.0, 10.0),
            Item::new("Helmet".to_string(), 0.0, 7.0),
            Item::new("Axe".to_string(), 15.0, 1.0),
            Item::new("Bow".to_string(), 8.0, 3.0),
            Item::new("Staff".to_string(), 5.0, 5.0),
            Item::new("Boots".to_string(), 0.0, 4.0),
            Item::new("Potion".to_string(), 0.0, 0.0), 
            Item::new("Tome".to_string(), 5.0, 5.0),
            Item::new("Dagger".to_string(), 12.0, 1.0),
            Item::new("Crossbow".to_string(), 10.0, 2.0),
            Item::new("Mace".to_string(), 14.0, 0.0),
            Item::new("Spear".to_string(), 9.0, 3.0),
            Item::new("Wand".to_string(), 6.0, 4.0),
            Item::new("Staff". to_string(), 7.0, 3.0),
            Item::new("Orb".to_string(), 9.0, 6.0),
        ];
        Self { items }
    }

    pub fn get_random_item(&self) -> Item {
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..self.items.len());
        self.items[index].clone()
    }

    pub fn add_item(&mut self, item: Item) {
        self.items.push(item);
    }
}