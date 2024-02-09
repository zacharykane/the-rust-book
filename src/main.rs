use ferris_says::say;
use rand::Rng;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::io::{stdin, stdout, BufWriter};

mod printing;

fn main() {
    printing::public_print();

    let message = String::from("Hello fellow Rustaceans!");

    let width = message.chars().count();
    let stdout = stdout();
    let writer = BufWriter::new(stdout.lock());
    say(&message, width, writer).unwrap();

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Guess the secret number.");

        let mut guess = String::new();

        stdin().read_line(&mut guess).expect("failed to read line!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let result = match guess.cmp(&secret_number) {
            Less => "Your guess was too small",
            Equal => "You win!!!",
            Greater => "Your guess was too big",
        };

        println!("You guessed {}. {}", guess, result);

        if guess == secret_number {
            break;
        }
    }
}
