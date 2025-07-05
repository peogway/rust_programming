#[derive(Clone)]
pub struct Item {
    name: String, 
    damage: f32,
    defense: f32,
}

impl Item {
    pub fn new(name: String, damage: f32, defense: f32) -> Self {
        Self { name, damage, defense }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_damage(&self) -> f32 {
        self.damage
    }

    pub fn get_defense(&self) -> f32 {
        self.defense
    }

    pub fn set_damage(&mut self, damage: f32) {
        self.damage = damage;
    }

    pub fn set_defense(&mut self, defense: f32) {
        self.defense = defense;
    }
}