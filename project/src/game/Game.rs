use crate::gear::Item::*;
use crate::gear::Items::*;
use crate::player::Player::*;
use crate::monster::Boss::*;

use std::io;



use rand::Rng;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;



pub fn read_all_stdin_nonblocking() {
    let mut input = String::new();
    println!("\nPlease Press 'enter' several times to consume your leftover inputs...");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
}

pub fn found_weapon(player: &mut Player, item: Item){
    let mut input = String::new();
    let max_len = item.get_name().len().max(player.get_item().get_name().len());
    println!("Currently you have\t{:width$}\twith {:>2}🗡️  and {:>2}🛡️.", player.get_item().get_name(), player.get_item().get_damage(), player.get_item().get_defense(), width = max_len);
    println!("You found a       \t{:width$}\twith {:>2}🗡️  and {:>2}🛡️.", item.get_name(), item.get_damage(), item.get_defense(), width = max_len);
    println!("Do you want to change? (y/n)");
    io::stdin()
        .read_line(&mut input)
        .expect("Fail to read line");
    let option = match input.trim().to_lowercase().as_str() {
        "y" => {
            player.set_item(item);
            println!("You changed your weapon.\n");
        }
        "n" => {
            println!("Your weapon stays unchanged.\n");
        }
        _ => {
            println!("Invalid input. Defaulting not to changing weapon.");
        }
    };
}

pub fn count_down(n: u8) {
    thread::spawn(move || {
        let mut i = n;
        while i > 0 {
            thread::sleep(Duration::from_secs(1));
            println!("{}",i);
            i -= 1;
        }

    }).join().expect("Thread failed");
    println!();
}

enum Message {
    Count(u32),
    ZeroB,
    Timeout,
}
pub fn gold_stage() -> u32{
    let mut input = String::new();
    println!("\nYou hit gold stage 💰💰💰!!!");
    println!("The more you type 'b', the more money 🪙  you get!\n");
    loop {
        println!("You have 10 seconds type as much 'b' as you can. Are you ready? (y/n)");
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");
        match input.trim().to_lowercase().as_str() {
            "y" => {
                println!("Let's start in 5 seconds!!!");
                count_down(5);
                println!("Let's gooo !!!");
                break;
            }
            "n" => {
                println!("Take your time. 5 seconds cool down.");
                count_down(5);
            }
            _ => {
                println!("Invalid input. Defaulting to be ready. Starting in 5 seconds.");
                count_down(5);
                println!("Let's gooo !!!\n");
                break;
            }
        }
    }
    let (tx, rx) = mpsc::channel();
    let (tx_time, rx_time) = mpsc::channel();
    let tx1 = tx.clone();
    thread::spawn(move || {
        let mut ip = String::new();
        loop {
            ip.clear();
            if rx_time.try_recv().is_ok() {
                return;
            }
            if io::stdin().read_line(&mut ip).is_err() {
                return;
            }
            let user_input = ip.trim().to_lowercase();
            let c = user_input.chars().filter(|&ch| ch == 'b').count() as u32;
    
            let res = if c == 0 {
                tx1.send(Message::ZeroB)
            } else {
                tx1.send(Message::Count(c))
            };

            if res.is_err(){
                return;
            }
        }
    });
    

    let tx2 = tx.clone();
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(10));
        tx2.send(Message::Timeout).unwrap();
    });
    let mut count = 0;
    for received in rx {
        match received {
            Message::Count(n) => {
                count += n * 10;
                println!("You typed {} letter 'b', received {}🪙  !", n, n * 10);
            }
            Message::ZeroB => {
                println!("You typed 0 letter 'b', received 0🪙  !");
            }
            Message::Timeout => {
                println!("You received {}🪙  in total!\n", count);
                let _ = tx_time.send(());
                read_all_stdin_nonblocking();
                return count as u32;
            }
        }
    }

    count as u32
}


pub fn found_treasure(player: &mut Player) {
    let items = Items::new();
    let item = rand::thread_rng().gen_range(1..=7);
    let coin = rand::thread_rng().gen_range(20..=30);

    match item {
        1 => {
            println!("You found a health potion! You gain 20 HP.\n");
            player.add_hp(20.0);
        }
        2 => {
            println!("You found a damage potion! You gain 5 damage.\n");
            player.add_damage(5.0);
        }
        3 => {
            println!("You found a defense potion! You gain 5 defense.\n");
            player.add_defense(5.0);
        }
        4 => {
            println!("You found {} 🪙\n", coin);
            player.add_coins(coin);
        }
        5 => {
            let item = items.get_random_item();
            found_weapon(player, item);
        }
        _ => {
            println!("The treasure is empty. You found nothing.\n");    
        }
    }
}

pub fn player_turn(player: &mut Player, boss: &mut Boss, diff_multi: f32, stage: u8){
    println!("{}", player.get_info());
    println!("Your turn!");
    println!("{} - 🩸 {}", boss.get_emo(), boss.get_hp());
    println!("Type 'attack' in 5 seconds to attack the boss", );
    let (tx, rx) = mpsc::channel();
    let (tx_time, rx_time) = mpsc::channel();
    let tx1 = tx.clone();
    thread::spawn(move || {
        let mut input = String::new();
        loop {
            input.clear();
            if rx_time.try_recv().is_ok() {
                return;
            }
            if io::stdin().read_line(&mut input).is_err() {
                return;
            }
            if input.trim().to_lowercase().as_str() == "attack" {
                let res = tx1.send(1);
                if res.is_err(){
                    return;
                }
                break;
            } else {
                if rx_time.try_recv().is_ok() {
                    return;
                } 
                println!("Invalid input, please type 'attack'.");
            }
        }
    });
    let tx2 = tx.clone();
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(5));
        let res = tx2.send(0);
        if res.is_err(){
            return;
        }
    });
    
    let choice: i32 = rx.recv().unwrap();


    if choice == 0 {
        println!("You miss your hit!\n");
        let _ = tx_time.send(());
        return;

    } else if choice == 1 {
        // Player attacks boss
        let attack_damage = ((player.get_damage() * player.get_damage()) / boss.get_defense() * 10.0).round() / 10.0;
        println!("The boss've lost {} 🩸!", attack_damage);
        boss.decrease_hp(attack_damage);
        let _ = tx_time.send(());
    }
}

pub fn boss_turn(player: &mut Player, boss: &mut Boss, diff_multi: f32, stage: u8){
    // Boss attacks back
    println!("{}", player.get_info());

    println!("Boss turn!");
    println!("{} - 🩸 {}", boss.get_emo(), boss.get_hp());
    println!("In 5 seconds, type 'dodge' to dodge, or 'block' to block the attack.");
    let (tx, rx) = mpsc::channel();
    let (tx_time, rx_time) = mpsc::channel();
    let tx1 = tx.clone();
    thread::spawn(move || {
        let mut input = String::new();
        loop {
            input.clear();
            if rx_time.try_recv().is_ok() {
                return;
            }
            if io::stdin().read_line(&mut input).is_err() {
                return;
            }
            match input.trim().to_lowercase().as_str() {
                "dodge" => {
                    let res = tx1.send(1);
                    if res.is_err(){
                        return;
                    }
                    break;
                }
                "block" => {
                    let res = tx1.send(2);
                    if res.is_err(){
                        return;
                    }
                    break;
                }
                _ => {
                    if rx_time.try_recv().is_ok() {
                        return;
                    }
                    println!("Invalid input, please type 'dodge' or 'block'.");
                }
            }
        }
    });
    let tx2 = tx.clone();
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(5));
        let res = tx2.send(0);
        if res.is_err(){
            return;
        }
    });

    let choice: i32 = rx.recv().unwrap();
    if choice == 0 {
        println!("You are too slow, receiving damage!");
        let block_amount = ((boss.get_damage()*100.0/(100.0+player.get_defense()) + diff_multi*stage as f32)*10.0).round()/10.0;
        if block_amount <= 0.0 {
            println!("Your defense is higher, block all damage\n");
        } else {
            player.decrease_hp(block_amount);
            println!("The boss attacks you for {} 🩸 !\n", block_amount);
        }
        let _ = tx_time.send(());
        return;
    } else if choice == 1 {
        // Player dodges
        println!("You successfully dodged the attack!\n");
        let _ = tx_time.send(());
        return;
    } else if choice == 2 {
        // Player blocks
        let block_amount = boss.get_damage() - player.get_defense() + diff_multi*stage as f32;
        if block_amount <= 0.0 {
            println!("Your defense is higher, block all damage\n");
        } else {
            player.decrease_hp(block_amount);
            println!("You blocked the attack, but still lost {} 🩸 !\n", block_amount);
        }
        let _ = tx_time.send(());
        return;
    }
}

pub fn intro(){
    println!("\n\n\n\n\n");
    println!("==========================================================================");
    println!("||       __                          __      ___      __                ||");
    println!("||     |    \\    |    |   |\\  |    /    \\   |       /    \\    |\\  |     ||");
    println!("||     |     |   |    |   | \\ |   |    __   |---   |      |   | \\ |     ||");
    println!("||     | __ /    | __ |   |  \\|    \\ __ |   |___    \\ __ /    |  \\|     ||");
    println!("||                                                                      ||");
    println!("==========================================================================\n");
}

pub fn greet() -> String{
    // Name
    let mut input = String::new();
    println!("Please enter your name:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let player_name = input.trim().to_string();
    input.clear();

    println!();
    println!("HELLO, {}! PREPARE FOR YOUR ADVENTURE!", player_name);
    println!();

    player_name
}

pub fn mode(mode_string: &mut String) -> i32 {
        // Mode
        let mut  input = String::new();
        println!("Choose your game mode:");
        println!("1) Easy   🐣   - Enemies are weaker, you have more health");
        println!("2) Normal 🎯   - Balanced difficulty");
        println!("3) Hard   💀   - Enemies are stronger, you have less health");
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mode = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input, defaulting to Normal mode.");
                2
            }
        };
        input.clear();
        match mode {
            1 => {
                println!("You have chosen Easy mode!");
                *mode_string = "EASY 🐣".to_string();
            },
            2 => {
                println!("You have chosen Normal mode!");
                *mode_string = "NORMAL 🎯".to_string();
            },
            3 => {  
                println!("You have chosen Hard mode!");
                *mode_string = "HARD 💀".to_string();
            },
            _ => {
                println!("Invalid choice, defaulting to Normal mode.");
                *mode_string = "NORMAL 🎯".to_string();
            }
        };
        println!();

        mode
}

pub fn player_init(player_name: String, diff_multi: f32) -> Player {
        // Character
        let mut input = String::new();
        println!("Please choose your character:");
        println!("1) Warrior - High damage, low defense");
        println!("2) Mage    - Medium damage, medium defense");
        println!("3) Rogue   - Low damage, high defense");
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut character_choice: u8 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input, defaulting to Warrior.");
                1
            }
        };
        let mut player = match character_choice {
            1 => {
                println!("You have chosen Warrior!");
                // Name, HP, Damage, Defense, Level, Experience, Coins, Item
                Player::new(player_name.to_string(), 100.0 - 60.0*diff_multi, 20.0, 5.0, 1, 0, 0, Item::new("Sword".to_string(), 10.0, 2.0), 1)
            }
            2 =>{
                println!("You have chosen Mage!");
                // Name, HP, Damage, Defense, Level, Experience, Item
                Player::new(player_name.to_string(), 80.0 - 60.0*diff_multi, 15.0, 10.0, 1, 0, 0, Item::new("Orb".to_string(), 9.0, 6.0), 1) 
            }
            3 =>{ 
                println!("You have chosen Rogue!");
                // Name, HP, Damage, Defense, Level, Experience, Item
                Player::new(player_name.to_string(), 90.0 - 60.0*diff_multi, 10.0, 15.0, 1, 0, 0, Item::new("Dagger".to_string(), 12.0, 1.0), 1) 
            }
            _ => {
                println!("Invalid choice, defaulting to Warrior.");
                // Name, HP, Damage, Defense, Level, Experience, Item
                Player::new(player_name.to_string(), 100.0 - 60.0*diff_multi, 20.0, 5.0, 1, 0, 0, Item::new("Sword".to_string(), 10.0, 2.0), 1) 
            }
        };

        player
}

pub fn shopping_stage(player: &mut Player ){
    let mut input = String::new();
    let items = Items::new();
    println!("You have reached a shopping stage! You can buy items here.");
    loop {
        println!("You currently have {}🪙.\n", player.get_coins());
        println!("1) Buy a health potion  (20 coins, increase 20🩸).");
        println!("2) Buy a damage potion  (30 coins, increases 5🗡️).");
        println!("3) Buy a defense potion (30 coins, increases 5🛡️).");
        println!("4) Buy a random weapon  (50 coins, test your luck).");
        println!("5) Buy an extra life 👼 (1000 coins).");
        println!("6) Stop shoping.");
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let choice: u8 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input, defaulting to stopping shopping.");
                7
            }
        };
        match choice {
            1 => {
                if player.get_coins() >= 20 {
                    player.set_coins(player.get_coins() - 20);
                    player.heal(20.0);
                    println!("You bought a health potion! Your HP is now {}🩸.\n", player.get_hp());

                } else {
                    println!("You don't have enough coins to buy a health potion.\n");
                }
            },
            2 => {
                if player.get_coins() >= 30 {
                    player.set_coins(player.get_coins() - 30);
                    player.add_damage(5.0);
                    println!("You bought a damage potion! Your damage is now {}🗡️.\n", player.get_damage());

                } else {
                    println!("You don't have enough coins to buy a damage potion.\n");
                }
            },
            3 => {
                if player.get_coins() >= 30 {
                    player.set_coins(player.get_coins() - 30);
                    player.add_defense(5.0);
                    println!("You bought a defense potion! Your defense is now {}🛡️.\n", player.get_defense());

                } else {
                    println!("You don't have enough coins to buy a defense potion.\n");
                }
            },
            4 => {
                if player.get_coins() >= 50 {
                    player.set_coins(player.get_coins() - 50);
                    println!("You bought a random weapon.\n");
                    let item = items.get_random_item();
                    found_weapon(player, item);

                } else {
                    println!("You don't have enough coins to buy a random weapon.\n");
                }
            },
            5 => {
                if player.get_coins() >= 1000 {
                    player.set_coins(player.get_coins() - 1000);
                    player.set_lives(player.get_lives() + 1);
                    println!("You bought an extra life.\n");

                } else {
                    println!("You don't have enough coins to buy an extra life.\n");
                }
            },
            6 => {
                println!("Stopping shopping!\n");
                break;
            },
            _ => {
                println!("Invalid choice, stopping shopping.\n");
                break;
            }
        }
    }
}

