use crate::game::Country::{Country};

pub struct GameMap {
    countries: Vec<Country>,
}

impl GameMap {
    pub fn new () -> Self {
        let countries = vec![
            Country::new("Denmark".to_string(), 6000000, 50000, vec![], false),
            Country::new("Finland".to_string(), 5600000, 900000, vec![], false),
            Country::new("Norway".to_string(), 5500000, 100000, vec![], false),
            Country::new("Sweden".to_string(), 10000000, 200000, vec![], false),
        ];
        Self {
            countries,
        }
    }

    pub fn get_country_by_index(&mut self, index : usize) -> &mut Country {
        &mut self.countries[index]
    }

    pub fn list_countries(&self){
        for (i, country) in self.countries.iter().enumerate() {
            println!("{}) {}", i+1, country.get_name());
        }
    }

    pub fn get_countries(&self) -> &Vec<Country> {
        &self.countries
    }

    pub fn set_countries(&mut self, countries: Vec<Country>) {
        self.countries = countries;
    }

    pub fn other_countries_turn(&mut self, name: &String) {
        for country in &mut self.countries {
            if country.get_name() != name && !country.is_conquered() {
                country.add_army();
            }
        }
    }
    

}