use crate::gear::Item::{Item};
pub struct Player {
    name: String,
    hp: f32,
    damage: f32,
    defense: f32,
    lvl: u8,
    exp: u32,
    coins: u32,
    item: Item,
    lives: u8,
}


impl Player {
    pub fn new(name: String, hp: f32, damage: f32, defense: f32, lvl: u8, exp: u32, coins: u32, item: Item, lives: u8) -> Self {
        Self { name, hp, damage, defense, lvl, exp, coins, item, lives }
    }

    pub fn get_info(&self) -> String {
        format!(
            "🩸 {}\t🗡️  {}\t🛡️  {}\tWeapon: {}\t🪙  {} \tLvl: {}\tExp: {}/100\t❤️  : {}\n",
            self.hp, self.damage, self.defense, self.item.get_name(), self.coins, self.lvl, self.exp, self.lives
        )
    }

    pub fn get_lives(&self) -> u8 {
        self.lives
    }

    pub fn set_lives(&mut self, num: u8){
        self.lives = num;
    }

    pub fn get_attack(&mut self, attack_damage: f32) {
        let damage_recieve = attack_damage - self.defense;
        if damage_recieve <= 0.0{
            println!("Your defense is higher. You blocked all damage!");
            println!("Bonus 20 exp and 50🪙  !\n ");
            self.coins += 50;
            self.exp += 20;
        }else {
            println!("The monster dealt {}🩸 to you.\n", damage_recieve);
            self.hp -= damage_recieve
        }
    }

    pub fn get_item(&self) -> Item {
        self.item.clone()
    }

    pub fn set_item(&mut self, item: Item) {
        self.damage -= self.item.get_damage();
        self.defense -= self.item.get_defense();
        self.item = item.clone();
        self.damage += item.get_damage();
        self.defense += item.get_defense();
    }

    pub fn add_exp(&mut self, exp: u32) {
        self.exp += exp;
        // Check for level up
        while self.exp >= 100 * self.lvl as u32 {
            self.lvl += 1;
            self.hp += 10.0; // Increase HP on level up
            self.damage += 2.0; // Increase damage on level up
            self.defense += 1.0; // Increase defense on level up
            self.exp -= 100; // Reset experience after leveling up
            println!("You leveled up! You are now level {}! Your stats are increased! (🩸 🡅 10.0\t🗡️  🡅 2.0\t🛡️ 🡅 1.0)\n", self.lvl);
        }
    }
    pub fn add_damage(&mut self, damage: f32) {
        self.damage += damage;
    }

    pub fn add_defense(&mut self, defense: f32) {
        self.defense += defense;
    }

    pub fn add_hp(&mut self, hp: f32) {
        self.hp += hp;
        if self.hp < 0.0 {
            self.hp = 0.0; // Ensure HP doesn't go below 0
        }
    }

    pub fn decrease_hp(&mut self, damage: f32) {
        self.hp -= damage;
        if self.hp < 0.0 {
            self.hp = 0.0; // Ensure HP doesn't go below 0
        }
    }

    pub fn heal(&mut self, amount: f32) {
        self.hp += amount;
    }

    pub fn decrease_damage(&mut self, damage: f32) {
        self.damage -= damage;
        if self.damage < 0.0 {
            self.damage = 0.0; // Ensure damage doesn't go below 0
        }
    }

    pub fn decrease_defense(&mut self, defense: f32) {
        self.defense -= defense;
        if self.defense < 0.0 {
            self.defense = 0.0; // Ensure defense doesn't go below 0
        }
    }

    pub fn add_coins(&mut self, coins: u32) {
        self.coins += coins;
    }

    pub fn get_coins(&self) -> u32 {
        self.coins
    }
    pub fn set_coins(&mut self, coins: u32) {
        self.coins = coins;
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

    pub fn get_lvl(&self) -> u8 {
        self.lvl
    }

    pub fn get_exp(&self) -> u32 {
        self.exp
    }

    pub fn set_hp(&mut self, hp: f32) {
        self.hp = hp;
    }

    pub fn set_damage(&mut self, damage: f32) {
        self.damage = damage;
    }

    pub fn set_defense(&mut self, defense: f32) {
        self.defense = defense;
    }

    pub fn set_lvl(&mut self, lvl: u8) {
        self.lvl = lvl;
    }

    pub fn set_exp(&mut self, exp: u32) {
        self.exp = exp;
    }
}