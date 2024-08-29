use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    loop {
        println!("Guess the number!");

        let secret_number = rand::thread_rng().gen_range(1..=100);

        println!("The secret number is: {secret_number}");

        println!("Please input your guess!");

        let mut guess = String::new(); // mutable value, defaults are immutables

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Aah! Looks like you put some invalid input. Try again!");
                continue
            },
        };
        // guess = "Updated Value".to_string(); // because it is mutable
        println!("You guessed: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }

    // let x = 5;
    // let y = 8;
    // println!("x is {x} and y + 2 = {}", y + 2);
}
