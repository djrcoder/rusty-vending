use std::cmp::Ordering;
use std::io;

fn main() {
    // Accept coins to get money from the customer

    let valid_coins: [i32; 4] = [10, 20, 50, 100];

    // let wallet = unsigned

    loop {
        println!("Input coins. Snacks cost £1");

        let mut coin_flag: bool = false;
        let mut coin_input = String::new();

        io::stdin()
            .read_line(&mut coin_input)
            .expect("Failed to read line.");

        let coin_input: i32 = match coin_input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        for coin in valid_coins.iter() {
            if coin_input == *coin {
                println!("valid coin");
                coin_flag = true;
                // Coin is valid, add to wallet
                // Once wallet is £1 in value, next stage
                break;
            }
        }

        // if flag is no then print

        if coin_flag == false {
            println!("invalid coin");
            // Empty wallet, gibe change, start over
        }
    }
}
