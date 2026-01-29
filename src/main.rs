use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    loop {
        println!("===================");
        let secret_number: i32 = rand::rng().random_range(1..=10);
        let mut guess: String = String::new();

        println!("Please enter your guess:");

        io::stdin().read_line(&mut guess).expect("there was a problem while getting input");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("Failed conversion to i32: {e}");
                continue;
            }
        };

        println!("You guess {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You win!"),
        }
        println!("The secret number is: {}", secret_number);

        println!("===================");

        if guess < 1 {
            break;
        }
    }
}
