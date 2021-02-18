#![allow(unused_variables)]
#![allow(unused_variables)]
#![allow(unused_mut)]
use std::collections::HashMap;
// print anything and multiple
macro_rules! ll {
  ($( $line:expr),* $(,)?) => {
      $( println!("{:#?}", $line); )*
  };
}

fn translate_letter(letter: &char, shifter: usize) -> char {
  let mut alpha: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
  let mut find_alpha = HashMap::new();
  find_alpha.extend((0..26).map(|num| (alpha[num], num)));
  // ll!(find_alpha.get(&'h').unwrap()); // Some(val) is returned, why we need unwrap https://doc.rust-lang.org/rust-by-example/error/option_unwrap.html

  match find_alpha.get(&letter) {
    Some(&index) => alpha[(index + shifter) % 26],
    _ => *letter
  }
}

fn main() {
  translate_letter(&'z', 1);
  let s = String::from("hello world!");
  let strings: Vec<char> = s.chars().collect();
  let mut new_string = String::from("");
  for letter in strings.iter() {
    new_string.push(translate_letter(&letter, 1));
  }
  ll!(new_string);
}
