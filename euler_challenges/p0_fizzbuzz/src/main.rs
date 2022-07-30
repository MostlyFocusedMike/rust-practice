// use std::collections::HashMap;
// print anything and multiple
macro_rules! ll {
    ($( $line:expr),* $(,)?) => {
        $( println!("{:#?}", $line); )*
    };
}

enum FizzyTypes {
    FizzBuzz,
    Fizz,
    Buzz,
    Other
}

fn cmp(number: i8) -> FizzyTypes {
    if  number % 3 == 0 && number % 5 == 0 { FizzyTypes::FizzBuzz }
    else if number % 3 == 0  { FizzyTypes::Fizz }
    else if number % 5 == 0  { FizzyTypes::Buzz }
    else { FizzyTypes::Other }
}

fn main() {
    let all_nums: Vec<i8> = (1..20).collect();
    for i in all_nums.iter() {
        if i % 3 == 0 && i % 5 == 0 {
            println!("Fizz Buzz {}", i)
        } else if i % 3 == 0 {
            println!("Fizz {}", i)
        } else if i % 5 == 0 {
            println!("Buzz {}", i)
        } else {
            ll!(i);
        }
    }

    // doing it with match and enums for fun
    for i in all_nums.iter() {
        match cmp(*i) {
            FizzyTypes::FizzBuzz=> println!("Fizz Buzz {}", i),
            FizzyTypes::Fizz=> println!("Fizz Buzz {}", i),
            FizzyTypes::Buzz=> println!("Buzz {}", i),
            FizzyTypes::Other=> println!("{}", i),
        }
    }
}
