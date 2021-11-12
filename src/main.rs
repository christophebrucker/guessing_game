// calling the external crate rand to use a random number generator
extern crate rand;

// bring the io library into scope
use std::io;

// calling the external crate ordering allowing to order numbers
use std::cmp::Ordering;

// calling the external crate Rng allowing to handle numbers ranges
use rand::Rng;

// defining the main function of our main.rs file
fn main() {

    // showing to the user a string
    println!("Guess an integer between 1 and 100!");

    // generate a random integer between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .ok()
            .expect("failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => {
                println!("You win!");
                break;
            }
        }
    }
}
