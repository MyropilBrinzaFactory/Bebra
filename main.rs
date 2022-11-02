use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    let mut count_try = 10;


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

        count_try = count_try - 1;

        if count_try == 0 {
            println!("The attempts are over, you lost =(");
            println!("Exit or try again? [1] Yes, [2] No");

            let mut choice: String = String::new();

            io::stdin()
                .read_line(&mut choice)
                .expect("Failed");

            let choice: u8 = choice.trim().parse().expect("Failed");
            
            if choice == 1 {
                break;
            } else {
                count_try = 10;
                continue;
            }
        } else {
            println!("Attempts left: {count_try}");
        }
    }
}