// #![allow(unused_variables)]
// #![allow(unused_variables)]
// #![allow(unused_mut)]
use std::collections::HashMap;
use std::convert::TryInto;
use std::io;

fn letter_translate(letter: &char, shifter: usize) -> char {
  let alpha: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
  let mut find_alpha = HashMap::new();
  find_alpha.extend((0..26).map(|num| (alpha[num], num)));

  match find_alpha.get(&letter) {
    Some(&index) => alpha[(index + shifter) % 26],
    _ => *letter
  }
}

fn message_translate(message: &str, shifter: isize) -> String {
  let temp_shifter = if shifter < 0 { 26 + shifter } else { shifter };
  let final_shifter: usize = temp_shifter.try_into().unwrap();

  let mut new_string = String::from("");
  let characters: Vec<char> = message.chars().collect();
  for letter in characters.iter() {
    new_string.push(letter_translate(&letter, final_shifter));
  }
  new_string
}

fn main() {
  let mut guess: String = String::new();
  io::stdin().read_line(&mut guess).expect("Failed to read line");

  let message = guess.to_lowercase();
  let encoded = message_translate(&message, 7);

  println!("\n\n\tHere is the encoded string: {}\n", encoded);
  for num in 1..11 {
    println!("Guess {}: {}", num, message_translate(&encoded, num * -1))
  }
}
