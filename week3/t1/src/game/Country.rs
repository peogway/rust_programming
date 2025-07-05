#[derive(Clone)]
pub struct Country {
    name: String,
    population: i64, 
    army_size: i64, 
    conquered_countries: Vec<String>,
    is_conquered: bool,
}

impl Country {
    pub fn new (name: String, population: i64, army_size: i64, conquered_countries: Vec<String>, is_conquered: bool) -> Self {
        Self {
            name,
            population,
            army_size,
            conquered_countries: conquered_countries,
            is_conquered: is_conquered,
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_population(&self) -> &i64 {
        &self.population
    }
    pub fn get_army_size(&self) -> &i64 {
        &self.army_size
    }

    pub fn is_conquered(&self) -> bool {
        self.is_conquered
    }

    pub fn set_population(&mut self, population: i64) {
        self.population = population;
    }
    pub fn set_army_size(&mut self, army_size: i64) {
        self.army_size = army_size;
    }

    pub fn set_is_conquered(&mut self, is_conquered: bool) {
        self.is_conquered = is_conquered;
    }

    pub fn get_conquered_nations(&mut self) -> &mut Vec<String> {
        &mut self.conquered_countries
    }

    pub fn set_conquered_nations(&mut self, name: String) {
        self.get_conquered_nations().push(name);
    }

    pub fn add_army(&mut self) {
        if self.get_population() - self.get_army_size() >= 50000 {
            self.army_size += 50000;
        }
    }
    
}

