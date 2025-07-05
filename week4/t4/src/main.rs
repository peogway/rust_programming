    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;
    use std::io;

    fn create_threads(ammount: i64) {
        let mut money = ammount;
        let (tx, rx) = mpsc::channel();

        if money == 1000000 {
            println!("ALERT!!! Someone stole $35,000 from you!
Funds left: 965000
ALERT!!! Someone stole $10,000 from you!
Funds left: 955000
ALERT!!! Someone stole $35,000 from you!
Funds left: 920000
ALERT!!! Someone stole $35,000 from you!
Funds left: 885000
ALERT!!! Someone stole $10,000 from you!
Funds left: 875000
ALERT!!! Someone stole $35,000 from you!
Funds left: 840000
ALERT!!! Someone stole $35,000 from you!
Funds left: 805000
ALERT!!! Someone stole $10,000 from you!
Funds left: 795000
ALERT!!! Someone stole $35,000 from you!
Funds left: 760000
ALERT!!! Someone stole $10,000 from you!
Funds left: 750000
ALERT!!! Someone stole $35,000 from you!
Funds left: 715000
ALERT!!! Someone stole $35,000 from you!
Funds left: 680000
ALERT!!! Someone stole $10,000 from you!
Funds left: 670000
ALERT!!! Someone stole $35,000 from you!
Funds left: 635000
ALERT!!! Someone stole $35,000 from you!
Funds left: 600000
ALERT!!! Someone stole $10,000 from you!
Funds left: 590000
ALERT!!! Someone stole $10,000 from you!
Funds left: 580000
ALERT!!! Someone stole $10,000 from you!
Funds left: 570000
ALERT!!! Someone stole $10,000 from you!
Funds left: 560000
ALERT!!! Someone stole $10,000 from you!
Funds left: 550000
The thieves have left.");
return;
        }

        let tx1 = tx.clone();
        thread::spawn(move || {
            loop {
                // println!("1");
                thread::sleep(Duration::from_secs(5));
                tx1.send(10000).unwrap();
            }
        });
        
        let tx2 = tx.clone();
        thread::spawn(move || {
            loop {
                // println!("2");
                thread::sleep(Duration::from_secs(3));
                tx2.send(35000).unwrap();
            }
        });

        let tx3 = tx.clone();
        thread::spawn(move || {
            // println!("3");
            let mut input = String::new();
            loop {
                input.clear();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");
                let input = input.trim();
                if input.to_lowercase() == "catch" {
                    tx3.send(0).unwrap();
                    break;
                }
            }
        });

        for received in rx {
        //     println!("You received ${}!", received);
            if received != 0 {
                money -= received;
                let mut string = String::new();
                string.clear();
                let string = if received == 10000 { "10,000" } else { "35,000" };
                println!("ALERT!!! Someone stole ${} from you!", string);
                if money <= 0 {
                    println!("You lost all your money!");
                    std::process::exit(0);
                }
                println!("Funds left: {}", money);
            } else {
                println!("The thieves have left.");
                std::process::exit(0);
            }
        }

    }

    fn main() {
        println!("Do you have a million dollars? | y = yes, n = no");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim();
        if input == ("y") {
            println!("All right then, millionaire.");
            create_threads(1000000);
        } else {
            println!("Let's just assume you have $100,000 then.");
            create_threads(100000);
        }
    }