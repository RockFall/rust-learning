use std::io::{self, Write};
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let mut try_again: String = String::new();

    while try_again != "n" {
        let max_guess_count: i32 = 10;
        let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

        println!("Guess the number!");

        for _ in 1..=max_guess_count {
            print!("Your guess: ");
            io::stdout().flush().unwrap();

            let mut guess: String = String::new();

            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");

            match guess.trim().parse::<u32>() {
                Ok(guess) => match guess.cmp(&secret_number) {
                    Ordering::Less => println!("Too small!"),
                    Ordering::Greater => println!("Too big!"),
                    Ordering::Equal => {
                        println!("You win!\n");
                        break;
                    }
                },
                Err(_) => println!("Please type a number!"),
            }
                
            
        }

        println!("The secret number is: {}", secret_number);

        
        println!("You ran out of guesses! :(\n");

        print!("Try again? (y/n): ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut try_again)
            .expect("Failed to read line");
    }
}
