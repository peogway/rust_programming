use crate::game::Country::{Country};

pub struct Player {
    country: Country,
}

impl Player {
    pub fn new(country: Country) -> Self {
        Self { country }
    }

    pub fn get_country(&mut self) -> &mut Country {
        &mut self.country
    }

    pub fn inspect(&self) {
        println!("Country information:");
        println!("Name: {}", self.country.get_name());
        println!("Population: {}", self.country.get_population());
        println!("Army size: {}", self.country.get_army_size());
    }

    pub fn conquer_nation(&mut self, target_country: &mut Country, name:  String) {
        if target_country.is_conquered() {
            println!("This country has already been conquered.");
            return;
        }
        if self.country.get_name() == target_country.get_name() {
            println!("Even your sick desires need boundaries.");
            return;
        }
        if self.country.get_army_size() == target_country.get_army_size() {
            println!("The war between {} and {} is a draw.", self.country.get_name(), target_country.get_name());
            return;
        }
        if self.country.get_army_size() < target_country.get_army_size() {
            println!("You have lost your war against {}. You have been conquered.", target_country.get_name());
            self.country.set_is_conquered(true);
            target_country.set_conquered_nations(self.country.get_name().clone());
            target_country.set_population(target_country.get_population() + self.country.get_population());
            target_country.set_army_size(target_country.get_army_size() + self.country.get_army_size());
            return;
        }
        println!("You have conquered {}", target_country.get_name());
        target_country.set_is_conquered(true);
        self.country.set_conquered_nations(target_country.get_name().clone());
        self.country.set_population(self.country.get_population() + target_country.get_population());
        self.country.set_army_size(self.country.get_army_size() + target_country.get_army_size());
    }
}