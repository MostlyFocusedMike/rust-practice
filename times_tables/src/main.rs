use rand::seq::SliceRandom;
use rand::thread_rng;
use std::io;

fn get_number() -> u32 {
    let mut guess: String = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    let guess: u32 = guess.trim().parse().expect("Please type a number!");
    return guess;
}

fn create_numbers() -> [u32; 8] {
    let mut rng = thread_rng();
    let mut y = [3, 4, 5, 6, 7, 8, 9, 12];
    y.shuffle(&mut rng);
    return y;
}

fn main() {
    let x_list = create_numbers();
    let y_list = create_numbers();
    let mut score = 0;
    // 'outer is a "label", https://doc.rust-lang.org/rust-by-example/flow_control/loop/nested.html
    // the ' is a lifetime? https://doc.rust-lang.org/nomicon/lifetimes.html
    'outer: for x in &x_list {
        for y in &y_list {
            println!("What is {} * {}? (enter 0 to exit)", x, y);
            let guess = get_number();
            if guess == x * y {
                score += 1;
                println!("Correct!");
            } else if guess == 0 {
                break 'outer;
            } else {
                println!("No, it's: {}", x * y);
            }
            if guess == 0 {
                println!("Correct!");
            }
        }
    }
    let point = if score == 1 { "point" } else { "points" };
    println!("Goodbye! You got {} {}", score, point)
}
