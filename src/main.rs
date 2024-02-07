use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    let mut chances: u32 = 20;

    loop {
        if chances == 0 {
            println!("Game over you have 0 chances left");
            break;
        }
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number from 1 to 100");
                continue;
            }
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                chances -= 1;
                println!("Too small! {} chances left", chances)
            }
            Ordering::Greater => {
                chances -= 1;
                println!("Too big! {} chances left", chances)
            }
            Ordering::Equal => {
                println!("You win! {} chances left", chances);
                break;
            }
        }
    }

    // Other ways to match the patterns with "match" and "control flow"

    // match guess {
    //     x if x > secret_number => println!("Too High"),
    //     x if x < secret_number => println!("Too Low"),
    //     x if x == secret_number => println!("You Won"),
    //     _ => println!("Unable to guess"),
    // }

    // if guess > secret_number {
    //     println!("Your guess was too high!");
    // }
    // if guess < secret_number {
    //     println!("Your guess was too low!");
    // }
    // if guess == secret_number {
    //     println!("Congratulations you guessed the secret number {secret_number}");
    // }
}
