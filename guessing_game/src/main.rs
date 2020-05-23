use rand::Rng; // Rng need to be here so we get traits, CH.10
use std::cmp::Ordering; // Ordering is an enum, CH 6
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 11);
    println!("Guess the number!");

    // unlimited guesses with loops
    loop {
        let mut guess = String::new();
        println!("Please input your guess.");

        // expect crashes the program and tells you why
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {}", guess);

        // we need the guess to be a number type so we use .parse
        // instead of expect for errors, we gracefully handle the situation
        // error handle is CH.9
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a number");
                continue;
            },
        };
        // multiple let is called "shadowing" described in ch 3

        // cmp compares two things
        // match is explained in ch 6
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; // exit the loop
            }
        }
    }
}
