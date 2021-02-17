const MAX_POINTS: u32 = 100_000;

fn main() {
  // PRINTING CONSOLE LOGS
  println!("Hello, world!");
  let name1 = "Mark";
  let name2 = "Stacey";
  println!("This is how interpolation works");
  println!("Here is {} and then there's {}", name1, name2);

  // RUST VARIABLES

  let x = 5;
  println!("The value of x is: {}", x);
  let mut y = 6;
  println!("The value of y is: {}", y);
  y = 7;
  println!("Now the value of y is: {}", y);


  // CONSTANTS
  println!("Notice how we can declare it in global");
  println!("And it still prints: {}", MAX_POINTS);
}
