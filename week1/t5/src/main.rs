use std::io;
use rand::Rng;

fn receive_player_attack_dmg() -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(12.5..=20.0)
}

fn receive_defense_multiplier() -> f32 {
    let mut rng = rand::thread_rng();
    1.0/rng.gen_range(2.0..=4.0)
}

fn receive_boss_attack_dmg() -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(5.0..=25.0)
}

fn main() {

    
    let mut your_hp : f32 = 100.0;
    let mut boss_hp = 150.0;
    let mut potions = 3;
    loop {
        if your_hp <= 0.0 {
            println!("You have been defeated!");
            break;
        }
        if boss_hp <= 0.0 {
            println!("You have defeated the boss!");
            break;
        }

        println!("| Your HP - {} | Boss HP - {} |", your_hp, boss_hp);
        println!("| 1) Attack | 2) Defend | 3) Heal |");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        let choice: u8 = match choice.trim().parse(){
            Ok(num) => num, 
            Err(_) => {
                println!("Invalid input, please enter a number between 1 and 3.");
                continue;
            }
        };
        let mut defense_multiplier: f32 = 1.0;
        let boss_attack_dmg = receive_boss_attack_dmg();
        match choice {
            1 => {
                let attack_dmg = receive_player_attack_dmg();
                boss_hp -= attack_dmg as f32;
                println!("Your attack deals {} amount of damage.", attack_dmg);
            },
            2 => {
                defense_multiplier = receive_defense_multiplier();
                println!("Defense activated!");
            },
            3 => {
                if potions > 0 {
                    your_hp += 25.0;
                    potions -= 1;
                    println!("You consume a potion.");
                } else {
                    println!("No potions left!");
                    continue;
    
                }
            },
            _ => {
                println!("Invalid choice, please select 1, 2, or 3.");
                continue;
            }
        }
        println!("You take {} damage.", 
                 (boss_attack_dmg * defense_multiplier) as f32);
        your_hp -= (boss_attack_dmg * defense_multiplier) as f32;
    }

}