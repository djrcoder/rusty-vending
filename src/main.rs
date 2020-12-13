// use std::cmp::Ordering;

use std::io;
#[derive(Debug)]

struct ProductToPurchase {
    name_of_product: String,
    price_of_product: u32,
    amount_of_product: u32,
}

struct VendingMachineContents {
    product1: ProductToPurchase,
    product2: ProductToPurchase,
}

fn main() {
    let crisps = ProductToPurchase {
        name_of_product: String::from("Ready Salted Crisps"),
        price_of_product: 100,
        amount_of_product: 10,
    };

    let drink = ProductToPurchase {
        name_of_product: String::from("Fanta"),
        price_of_product: 100,
        amount_of_product: 10,
    };

    let stocked_vending_machine = VendingMachineContents {
        product1: crisps,
        product2: drink,
    };

    println!(
        "Contents is: {:#?} and {:#?}",
        stocked_vending_machine.product1.name_of_product,
        stocked_vending_machine.product2.name_of_product
    );

    // Accept coins to get money from the customer

    let valid_coins: [i32; 4] = [10, 20, 50, 100];
    let mut total_input: i32 = 0;

    // let wallet = unsigned

    loop {
        println!("Input coins. Snacks cost £1");

        let mut valid_coin_flag: bool = false;
        let mut user_coin_input = String::new();

        io::stdin()
            .read_line(&mut user_coin_input)
            .expect("Failed to read line.");

        let user_coin_input: i32 = match user_coin_input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        for coin in valid_coins.iter() {
            if user_coin_input == *coin {
                println!("valid coin");

                valid_coin_flag = true;

                // Coin is valid, add to wallet

                total_input += coin;

                println!("Total amount is {}", total_input);
                // Once wallet is £1 in value, next stage
                break;
            }
        }

        // if flag is no then print

        if valid_coin_flag == false {
            println!("invalid coin");
            // Empty wallet, gibe change, start over
        }
    }
}
