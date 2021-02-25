mod lib;

use lib::GetNumber;

fn main() {
  let thing = GetNumber::new(10);


  println!("{:#?}", thing.value());
}