use std::io;
#[derive(Debug)]

struct Product {
    name_of_product: String,
    selection_code: u32,
    price_of_product: i32,
    amount_of_product: u32,
}

fn main() {
    let token_price: i32 = 100;
    let mut products = Vec::<Product>::new();
    let valid_coins: [i32; 4] = [10, 20, 50, 100];
    let mut total_input: i32 = 0;

    products.push(Product {
        name_of_product: String::from("Ready Salted Crisps"),
        selection_code: 1,
        price_of_product: 100,
        amount_of_product: 10,
    });

    products.push(Product {
        name_of_product: String::from("Fanta"),
        selection_code: 2,
        price_of_product: 100,
        amount_of_product: 10,
    });

    loop {
        println!("Input coins. Snacks cost 100 tokens");

        let mut valid_coin_flag: bool = false;
        let mut valid_selection_flag: bool = false;
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
                total_input += coin;

                println!("Total amount is {} tokens.", total_input);

                if total_input >= token_price {
                    println!("Products available");
                    for element in products.iter() {
                        println!(
                            "Selection code:{}, name:{}, price:{} ",
                            element.selection_code,
                            element.name_of_product,
                            element.price_of_product
                        );
                    }

                    println!("Make a selection");
                    let mut user_product_selection = String::new();

                    io::stdin()
                        .read_line(&mut user_product_selection)
                        .expect("Failed to read line.");

                    let user_product_selection: u32 = match user_product_selection.trim().parse() {
                        Ok(num) => num,
                        Err(_) => continue,
                    };

                    for element in products.iter() {
                        if user_product_selection == element.selection_code {
                            println!("Enjoy your {}", element.name_of_product);
                            total_input -= element.price_of_product;
                            println!("Returning {}", total_input);
                            total_input = 0;
                            valid_selection_flag = true;
                        }
                    }

                    if valid_selection_flag == false {
                        println!("Invalid Selection! Returning coins.");
                        total_input = 0;
                    }
                }
                break;
            }
        }

        if valid_coin_flag == false {
            println!("invalid coin! Returning coins.");
            total_input = 0;
        }
    }
}
