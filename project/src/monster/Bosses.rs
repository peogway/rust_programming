use rand::Rng;
use crate::monster::Boss::{Boss};

pub struct Bosses {
    bosses: Vec<Boss>,
}


impl Bosses {
    pub fn new () -> Self {
        let bosses = vec![
            Boss::new("Kragnar the Devourer".to_string(), 60.0, 30.0, 10.0, "🦖".to_string()),
            Boss::new("The Blighted King".to_string(), 80.0, 20.0, 20.0, "👺".to_string()),
            Boss::new("Ashen Wyrm".to_string(), 70.0, 40.0, 20.0, "🐉".to_string()),
            Boss::new("Soul Reaper Zarnok".to_string(), 70.0, 25.0, 5.0, "🧟".to_string()),
            Boss::new("Dreadmaw the Ancient".to_string(), 100.0, 15.0, 25.0, "🦂".to_string()),
        ];
        Self { bosses }
    }
    pub fn get_random_boss(&self) -> Boss {
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..self.bosses.len());
        self.bosses[index].clone()
    }


}
