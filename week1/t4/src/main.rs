use rand::Rng;

fn main(){
    fn receive_random() -> i16 {
        let mut rng = rand::thread_rng();
        rng.gen_range(1..=10)
    }

    fn measure_luck(luck: i32) -> String{ 
        if luck < 3 {
            return "LUCKY".to_string();
        } else {
            return "UNLUCKY".to_string();
        };
    }

    let mut counter: i32 = 0;
    loop {
        let num = receive_random();
        match num {
            1..=3 => {
                println!("Low...");
            },
            4..=6 => {
                println!("Middle!");
            },
            7..=9 => {
                println!("High!!");
            },
            10 => {
                println!("Jackpot!!!");
                break
            },
            _ => {
                println!("Uncovered");
                break;
            }
        }
        counter += 1;
    }
    let mut s = String::new();
    s.push_str(measure_luck(counter).as_str());
    println!("You are {}!", measure_luck(counter));
}