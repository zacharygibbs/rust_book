use std::io::stdin;
use::rand::{thread_rng, Rng};

pub fn main() {

    let rand_number = thread_rng().gen_range(1..=100);

    println!("Guess the number from 1 to 100!");

    let mut guess_count: isize = 1;
    while guess_count < 1000{
        let mut guess = String::new();

        println!("Please input your guess (Guess# {guess_count})");
        stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess_int: isize = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a valid number!");
                continue;
            }
        };
    //expect("Please type a number!");
        if (guess_int < 1) | (guess_int > 100) {
            println!("Guess must be between 1 and 100!");
            continue;
        } else if guess_int == rand_number {
            println!("Success - it only took you {guess_count} guesses!");
            break;
        } else if guess_int > rand_number {
            println!("Try Again guess too high!");
        } else {
            println!("Try Again guess too low!");
        }
        guess_count += 1;


    };
}
