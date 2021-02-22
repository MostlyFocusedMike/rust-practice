use std::io;
use std::collections::HashMap;

fn letter_translate(letter: &char, shifter: usize) -> char {
  let alpha: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
  let mut find_alpha = HashMap::new();
  find_alpha.extend((0..26).map(|num| (alpha[num], num)));

  match find_alpha.get(&letter) {
    Some(&index) => alpha[(index + shifter) % 26],
    _ => *letter
  }
}

enum Cyphers {
  V1,
  V2,
  V1Solve,
  V2Solve,
}

impl Cyphers {
  fn val(&self) -> [usize; 4] {
    match self {
        Cyphers::V1 => [1,2,3,4],
        Cyphers::V1Solve => [25,24,23,22],
        Cyphers::V2 => [7,8,9,10],
        Cyphers::V2Solve => [19,18,17,16],
    }
  }
}

fn get_input() -> u8 {
  println!("What Cypher would you like to use? 1 2");
  let mut input = String::new();
  io::stdin().read_line(&mut input).expect("Failed to read line");
  println!("You selected: {}", input);

  match input.trim().parse() {
      Ok(num) => num,
      Err(_) => {
          println!("No number entered, default selected");
          1
      },
  }
}

fn main() {
  let cypher;

  match get_input() {
    1 => cypher = Cyphers::V1,
    2 => cypher = Cyphers::V2,
    3 => cypher = Cyphers::V1Solve,
    _ => cypher = Cyphers::V2Solve,
  }

  let mut guess = String::new();
  io::stdin().read_line(&mut guess).expect("Failed to read line");

  let mut new_string = String::from("");

  for (i, letter) in guess.chars().enumerate() {
    let val = i % 4;
    let shifter = cypher.val()[val];

    new_string.push(letter_translate(&letter, shifter));
  }
  println!("{}", new_string);


}
