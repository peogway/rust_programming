pub mod game;
pub mod player;
pub mod gear;
pub mod monster;

use gear::Items::*;
use monster::Bosses::*;
use game::Game::*;

use std::io;
use std::io::Write;



use rand::Rng;



fn main() {
    let mut input = String::new();
    let mut stage = 1;
    let items = Items::new();
    let bosses = Bosses::new();   
    intro();  
    let player_name = greet();
    let mut mode_string = String::new();
    let mode = mode(&mut mode_string);
    let diff_multi = (mode - 2) as f32 *0.5;
    let mut player = player_init(player_name, diff_multi);
    println!("Starting the game with {} HP, {} damage, and {} defense.", player.get_hp(), player.get_damage(), player.get_defense());
    println!();
    println!("==========================================================================");
    loop {
        println!("STAGE {} - {}.\n", stage, mode_string);
        println!("{}", player.get_info());
        if stage %5 != 0 {

            // Shopping stage
            if stage % 3 == 0 { 
                shopping_stage(&mut player);
                stage += 1;
                continue;
            }

            // Gold stage
            if stage % 8 == 0 {
                stage += 1;
                player.add_coins(gold_stage());
                continue;
            }
            println!("Non-boss stage!\n");
            println!("1) Find treasure (probably gain items, coins, but no Exp).");
            println!("2) Fight a monster (gain items and Exp, coins, but lose HP).");
            println!("3) Skip stage (gain nothing).");
            input.clear();

            io::stdin().read_line(&mut input).expect("Failed to read line");
            let choice: u8 = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid input, defaulting to skipping stage.");
                    3
                }
            };
            match choice {
                1 => {
                    println!("You found a treasure! You gain a random item.");
                    found_treasure(&mut player);
                }
                2 => {
                    let monster_damage = rand::thread_rng().gen_range(5..=15) + 10 * diff_multi as i32; // Monster damage based on difficulty
                    player.get_attack(monster_damage as f32 + stage as f32*diff_multi);
                    if player.get_hp() <= 0.0 {
                        println!("You have been defeated by the monster! ☠️ ☠️ ☠️\n");
                        if player.get_lives() == 1 {
                            println!("You losed the dungeon at stage {}!", stage);
                            println!("Death is just a dark pause; your journey ends here… for now.\nFallen in the depths, but not forgotten.");
                            println!("Thank you for playing the game...");
                            return; // End the game if player is defeated
                        }
                        println!("You have been revived with 100🩸 .\n");
                        player.set_hp(100.0);
                        player.set_lives(player.get_lives() -1);
                    }
                    println!("You won a fight against a monster!");

                    println!("You have {}🩸 left.", player.get_hp());
                    println!("You gain 75 experience and 50 coins.");
                    player.add_exp(75);
                    player.add_coins(50);
                    println!("\nYou gain a random item.\n");
                    let item = items.get_random_item();
                    found_weapon(&mut player, item)
                    
                }
                3 => {
                    println!("You skipped the stage. Nothing happens.\n");
                    stage += 1;
                    continue;
                },
                _ => {
                    println!("Invalid choice, skipping stage.");
                    stage += 1;
                    continue;
                }
            }
        } else {
            println!("BOSS STAGE!\n");
            let mut boss = bosses.get_random_boss();
            if diff_multi >= 0.0 {
                boss.increase_hp(diff_multi*(stage*10) as f32);
            }else {
                boss.increase_hp(((stage-5)*10) as f32);
            }
            println!("\nYou encounter {} {}  (🩸 {})! Prepare for the battle!\n", boss.get_name(),boss.get_emo(), boss.get_hp());
            println!("Type 'attack' in your turn, or 'dodge', 'block' in boss turn.");
            println!("Press 'enter' to start the battle.");
            input.clear();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            println!("\nCounting 5 seconds down...");
            count_down(5);
            println!("Start!!!");

            loop {
                // attack
                player_turn(&mut player, &mut boss, diff_multi, stage);
                read_all_stdin_nonblocking();



                if boss.get_hp() <= 0.0 {
                    println!("You have defeated the {} {}! You gain 120 Exp, 150🪙 !!", boss.get_emo(), boss.get_name() );
                    player.add_exp(120);
                    player.add_coins(150);
                    
                    let num = rand::thread_rng().gen_range(1..=4);
                    println!("You found {} treasures!!!", num);
                    for i in 1..num{
                        print!("Press 'enter' to open chest {}: ", i);
                        io::stdout().flush().unwrap();
                        input.clear();
                        io::stdin()
                            .read_line(&mut input)
                            .expect("Failed to read line");
                        found_treasure(&mut player);
                    }

                    break;
                }

                println!("Boss turn in 3 seconds...");
                count_down(3);

                boss_turn(&mut player, &mut boss, diff_multi, stage);
                read_all_stdin_nonblocking();


                if player.get_hp() <= 0.0 {
                    println!("You have been defeated by the boss!");
                    if player.get_lives() == 1 {
                        println!("You losed the dungeon at stage {}!", stage);
                        println!("Death is just a dark pause; your journey ends here… for now.\nFallen in the depths, but not forgotten.");
                        println!("Thank you for playing the game...");
                        return; // End the game if player is defeated
                    }
                    println!("You have been revived with 100🩸 .\n");
                    player.set_hp(100.0);
                    player.set_lives(player.get_lives() -1);


                }

                println!("Your turn in 3 seconds...");
                count_down(3);
            }

        }
        if stage == 20 {
            println!("You have successfully conquer the dungeon!!! 👑 👑 👑");
            println!("The darkness fades. Peace returns.");
            println!("You stood alone. You stood victorious.");
            println!("Thank you for playing the game...");
        }
        stage += 1;
    }
}
