use rand::Rng;
use core::num;
use std::io;
use std::cmp::Ordering;

fn main() {
    
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {

        println!("Please input your guess:");

        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your guess: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("To small!"),
            Ordering::Greater => println!("To big!"),
            Ordering::Equal => { 
                println!("You win!");
                break;
            }
        }
    }
}
