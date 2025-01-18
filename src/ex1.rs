use std::io::stdin;
use std::cmp::Ordering;
use::rand::{thread_rng, Rng};

pub fn main() {

    let rand_number = thread_rng().gen_range(1..=100);

    println!("Guess the number from 1 to 100!");

    let mut guess_count = 1;
    while guess_count < 1000{
        let mut guess = String::new();

        println!("Please input your guess (Guess# {guess_count})");
        stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess_int: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                if guess.trim() == "quit" {
                    break;
                }
                println!("Please input a valid number!");
                continue;
            }
        };
    //expect("Please type a number!");
        if guess_int < 1 || guess_int > 100 {
            println!("Guess must be between 1 and 100!");
            continue;
        }
        match rand_number.cmp(&guess_int) {
            Ordering::Equal => {
                println!("Success - it only took you {} guesses!", &guess_count);
                break;
            },
            Ordering::Less => { // rand_number less than guess_int
                println!("Try Again guess too high!");
            },
            Ordering::Greater => { // rand_number greater than guess int
                println!("Try Again guess too low!");
            }

        };
        guess_count += 1;


    };
}
