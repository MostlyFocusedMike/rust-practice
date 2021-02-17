// print anything and multiple
macro_rules! ll {
    ($( $line:expr),* $(,)?) => {
        $( println!("{:#?}", $line); )*
    };
}

fn main() {
  let x = 0b1111_0000;
  ll!(x, [1,2,3]);
  println!("Hello, world! {}", x);
}