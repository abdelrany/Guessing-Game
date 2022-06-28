// use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::iter::repeat_with;

fn main() {
    println!("Guess the letter!");

    let secret_alphabet: String = repeat_with(fastrand::lowercase).take(10).collect();
    println!("The random alphabet is: {}", secret_alphabet);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: String = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_alphabet) {
            Ordering::Less => println!("Letter is too small!"),
            Ordering::Greater => println!("Letter is too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
