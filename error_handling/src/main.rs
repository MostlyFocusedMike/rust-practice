use std::fs::File;
use std::io::ErrorKind;

fn main() {
    println!("Hello, world!");

    // Panic with the given message
    // panic!("crash and burn");

    // RUST_BACKTRACE=1 cargo run
    // That will let you run rust with a backtrace so you can see what happened more clearly


    // Handling recoverable errors
    let f = File::open("hello.txt");

    let f = match f { // File::open returns a Result Enum (Ok(data) or Err(error))
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };


    // Getting more specific with your error handling with ErrorKind
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };

    // Even better use the closure function unrwap_or_else  |thing| is a closure
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });



    // unwrap and expect are shorter ways of dealing with Result, they return the Ok value, or just panic

    let f = File::open("hello.txt").unwrap(); // no error message on panic

    let f = File::open("hello.txt").expect("Failed to open hello.txt"); // specify error message on panic
}


use std::io;
use std::io::Read;


// You can also propogate errors up by returning them, and letting the parent function or functions generically handle them
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f { // verbose way, use ? operator
        Ok(file) => file,
        Err(e) => return Err(e), // explicit return
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) { // verbose way, use ? operator
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// this function does EXACTLY the same thing, but uses ? instead
// The ? can only be used on a function that returns a Result or Option Enum, with a value and an error
// the main() function is allowed to return that, by the way
fn read_username_from_file2() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// even MORE reduced by chaining methods
fn read_username_from_file3() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}