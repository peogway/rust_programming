pub mod game;

use game::Country::*;
use game::Player::*;
use game::GameMap::*;
use std::io;

fn main() {
    let mut fin = Country::new("Finland".to_string(), 5600000i64, 900000i64, vec![], false);
    let mut sweden = Country::new("Sweden".to_string(), 10000000i64, 200000i64, vec![], false);
    let mut norway = Country::new("Norway".to_string(), 5500000i64, 100000i64, vec![], false);
    let mut denmark = Country::new("Denmark".to_string(), 6000000i64, 50000i64, vec![], false);
    let mut countries = vec![&mut fin, &mut sweden, &mut norway, &mut denmark];
    let mut game_map = GameMap::new();

    println!("| 1) Finland | 2) Sweden | 3) Norway | 4) Denmark |");
    println!("Choose your country: ");
    let mut country_choice = String::new();
    io::stdin()
        .read_line(&mut country_choice)
        .expect("Failed to read line");
    let choice = country_choice.trim().parse::<usize>().unwrap_or(0);
    let mut player = Player::new(countries[choice - 1].clone());


    loop {

        println!("| Inspection on your own nation? | y = yes | n = no |");
    
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let inspect = input.trim().to_lowercase() == "y";
        if inspect {
            println!("An inspection has been completed..");
            player.inspect();
        } else {
            println!("The leader is confident. No inspection needed.");
        }
        input.clear();

        println!("| 1) Spy on a country | 2) Invade a country | 3) Expand military |");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read Line");

        let option = input.trim().parse();
        match option {
            Ok(2) => {
                game_map.list_countries();
                let mut target_country = String::new();
                io::stdin()
                    .read_line(&mut target_country)
                    .expect("Failed to read line");
                let target_index = target_country.trim().parse::<usize>().unwrap_or(0);
                let mut target_country = game_map.get_country_by_index(target_index - 1);
                let name = player.get_country().get_name().clone();
                player.conquer_nation(&mut target_country, name); 

            }
            Ok(1) => {
                game_map.list_countries();
                let mut target_country = String::new();
                io::stdin()
                    .read_line(&mut target_country)
                    .expect("Failed to read line");
                let target_index = target_country.trim().parse::<usize>().unwrap_or(0);
                if game_map.get_country_by_index(target_index - 1).get_name() == countries[choice - 1].get_name() {
                    println!("You can't spy on your own nation!");
                } else {
                    println!("Espionage successful.");
                    let pl = Player::new(game_map.get_country_by_index(target_index - 1).clone());
                    pl.inspect();
                }

            }
            Ok(3) => {
                player.get_country().add_army();
            }
            _ => {
                println!("Invalid game input. Try again.");
            }
        }
        // println!("{}", player.get_country().get_conquered_nations().join(", "));
        if player.get_country().is_conquered() {
            println!("Game over!");
            game_map.other_countries_turn(player.get_country().get_name());
            break;
        }
        let mut check = false;
        for country in game_map.get_countries() {
            if country.get_name() == player.get_country().get_name(){
                continue;
            }
            if !country.is_conquered() {
                check = true;
                break;
            }
        }
        if !check {
            println!("You have conquered all your targets. Good work, comrade!");
            game_map.other_countries_turn(player.get_country().get_name());
            break;
        }
    }
}
