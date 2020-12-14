// use std::cmp::Ordering;

use std::io;
#[derive(Debug)]

struct ProductToPurchase {
    name_of_product: String,
    selection_code: u32,
    price_of_product: i32,
    amount_of_product: i32,
}

struct VendingMachineContents {
    product1: ProductToPurchase,
    product2: ProductToPurchase,
}

fn main() {
    let crisps = ProductToPurchase {
        name_of_product: String::from("Ready Salted Crisps"),
        selection_code: 1,
        price_of_product: 100,
        amount_of_product: 10,
    };
    let drink = ProductToPurchase {
        name_of_product: String::from("Fanta"),
        selection_code: 2,
        price_of_product: 100,
        amount_of_product: 10,
    };
    let mut stocked_vending_machine = VendingMachineContents {
        product1: crisps,
        product2: drink,
    };

    // Accept coins to get money from the customer

    let valid_coins: [i32; 4] = [10, 20, 50, 100];
    let mut total_input: i32 = 0;

    // let wallet = unsigned

    loop {
        println!("Input coins. Snacks cost 100 tokens");

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
                // Once wallet is 100 in value, next stage
                if user_coin_input >= 100 {
                    // makeSelection(user_coin_input, stocked_vending_machine);
                    println!(
                        "Products available: Press {:#?} for {:#?}. Price {:#?}  and Press {:#?} for {:#?}. Price {:#?}",
                        stocked_vending_machine.product1.selection_code,
                        stocked_vending_machine.product1.name_of_product,
                        stocked_vending_machine.product1.price_of_product,
                        stocked_vending_machine.product2.selection_code,
                        stocked_vending_machine.product2.name_of_product,
                        stocked_vending_machine.product2.price_of_product,
                    );

                    println!("Make a selection");
                    let mut user_product_selection = String::new();

                    io::stdin()
                        .read_line(&mut user_product_selection)
                        .expect("Failed to read line.");

                    let user_product_selection: u32 = match user_product_selection.trim().parse() {
                        Ok(num) => num,
                        Err(_) => continue,
                    };

                    // loop
                    // if selection matches vending machine code
                    // dispense and wipe money

                    // for selection_code in stocked_vending_machine.product1.iter() {

                    if user_product_selection == stocked_vending_machine.product1.selection_code {
                        println!(
                            "Enjoy your {:#?} ",
                            stocked_vending_machine.product1.name_of_product
                        );
                        total_input -= stocked_vending_machine.product1.price_of_product;
                        stocked_vending_machine.product1.amount_of_product -= 1;
                    }

                    if user_product_selection == stocked_vending_machine.product2.selection_code {
                        println!(
                            "Enjoy your {:#?} ",
                            stocked_vending_machine.product2.name_of_product
                        );
                        total_input -= stocked_vending_machine.product2.price_of_product;
                        stocked_vending_machine.product2.amount_of_product -= 1;
                    }
                }

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
