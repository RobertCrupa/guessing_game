
use std::cmp::Ordering;
use std::io;
use rand::{thread_rng, Rng};
fn main() {
    println!("Guess the number!");

    let num: u8 = thread_rng()
                .gen_range(1..=100);

    loop {

        println!("Enter your guess:");

        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u8 = match guess.trim().parse() {
                                Ok(num) => num,
                                Err(_) => continue,
                            };

        println!("Got string: {guess}");

        match guess.cmp(&num) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("Winner!");
                break;
            }
        }
    }
}
