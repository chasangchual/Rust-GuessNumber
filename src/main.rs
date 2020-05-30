use std::io;
use rand::Rng; // random number generator
use std::cmp::*;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("Guess the number!");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {}", guess);

        let guess : i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You Win !!");
                break;
            },
            Ordering::Less => println!("Too small !!"),
            Ordering::Greater => println!("Too big !!")
        }
    }
}
