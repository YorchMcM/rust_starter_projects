use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    loop {

        println!("Input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(val) => val,
            Err(_) => {
                match guess.trim() {
                    "quit" => break,
                    "exit" => break,
                    "out" => break,
                    "fuera" => break,
                    "salir" => break,
                    "quita" => break,
                    _ => {
                        println!("You guessed: {}", guess.trim());
                        println!("That doesn't look like a number... Try again");
                        continue;
                    }
                }
            }
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

    }

}
