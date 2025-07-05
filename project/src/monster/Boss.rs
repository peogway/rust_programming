#[derive(Clone)]
pub struct Boss {
    name: String,
    hp: f32,
    damage: f32,
    defense: f32,
    emoji: String,
}

impl Boss {
    pub fn new(name: String, hp: f32, damage: f32, defense: f32, emoji: String) -> Self {
        Self { name, hp, damage, defense, emoji }
    }

    pub fn get_emo(&self) -> &String {
        &self.emoji
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_hp(&self) -> f32 {
        self.hp
    }

    pub fn get_damage(&self) -> f32 {
        self.damage
    }

    pub fn get_defense(&self) -> f32 {
        self.defense
    }

    pub fn decrease_hp(&mut self, damage: f32) {
        self.hp -= damage;
        if self.hp < 0.0 {
            self.hp = 0.0; // Ensure HP doesn't go below 0
        }
    }

    pub fn increase_hp(&mut self, hp: f32) {
        self.hp += hp;
    }

    pub fn heal(&mut self, amount: f32) {
        self.hp += amount;
    }
}