use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guesss the number between 0 and 100!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        // println!("The secret number is {} ", secret_number);

        println!("Input guess now: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Winner winner chicken dinner 🐔");
                break;
            }
        }
    }
}
