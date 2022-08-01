# Rust Notes
Here are my rust notes. Primarily based off the Book of Rust

[Rust Notes](#rust-notes)
- [Rust Notes](#rust-notes)
- [Ch.1 Getting Started](#ch1-getting-started)
  - [1.1 Installation](#11-installation)
  - [1.2 Hello Rust!](#12-hello-rust)
    - [Anatomy of a Rust Program](#anatomy-of-a-rust-program)
    - [Compiling and Running Are Separate Steps](#compiling-and-running-are-separate-steps)
    - [rust formatting](#rust-formatting)
  - [1.3 Hello, Cargo!](#13-hello-cargo)
    - [the Cargo.toml](#the-cargotoml)
    - [Building and Running a Cargo Project](#building-and-running-a-cargo-project)
  - [Running on save](#running-on-save)
- [Ch. 2: Programming a Guessing Game](#ch-2-programming-a-guessing-game)
  - [std:io](#stdio)
  - [Storing Values with Variables](#storing-values-with-variables)
  - [Using a Crate to Get More Functionality](#using-a-crate-to-get-more-functionality)
  - [Check the program](#check-the-program)
- [Ch.3 Common Programming Concepts](#ch3-common-programming-concepts)
  - [Variables](#variables)
    - [variables vs constants](#variables-vs-constants)
    - [Shadowing Variables](#shadowing-variables)
  - [Basic Data Types](#basic-data-types)
  - [Integer Types](#integer-types)
    - [Integer overflow](#integer-overflow)
  - [Floating Point](#floating-point)
  - [Numeric Operations](#numeric-operations)
  - [Booleans](#booleans)
  - [The char type](#the-char-type)
  - [String Slices / String Literals](#string-slices--string-literals)
  - [Compound Types](#compound-types)
  - [Tuples](#tuples)
  - [Arrays](#arrays)
  - [Functions](#functions)
- [Ch 4: Understanding ownership](#ch-4-understanding-ownership)
  - [Movement and copying](#movement-and-copying)
  - [Functions and , and returning values and scope](#functions-and--and-returning-values-and-scope)
  - [References and borrowing](#references-and-borrowing)
  - [Mutable References](#mutable-references)
  - [Dangling References](#dangling-references)
  - [Another book](#another-book)

# Ch.1 Getting Started

## 1.1 Installation
For Mac, just do
```
brew install rustup
```
`rustup` is the version manager for rust, like `nvm`.

you can update with:
```
rustup update
```

To uninstall Rust and rustup:

```plaintext
rustup self uninstall
```

To check the installation was successful, use
```
rustc --version
```

`rustc` is the compiler for rust

You can also see the offline docs with
```plaintext
rustup docs
```


## 1.2 Hello Rust!
- rust files use the `.rs` extension and need to be compiled

```rust
// file: main.rs
fn main() {
    println!("Hello, world!");
}
```

- to compile this file do:

```bash
rustc main.rs
# makes the executable `main`
ls
main main.rs

./main
Hello, world!
```
- "Rust is an ahead-of-time compiled language, meaning you can compile a program and give the executable to someone else, and they can run it even without having Rust installed"

### Anatomy of a Rust Program
- functions look like the above, but rust uses a special "main" funciton, it takes nothing and returns nothing. Needs to be top level (maybe)
- `println!` is not just a function, but a macro, you can tell because it uses the `!`. Regular functions don't have that
- Most lines in rust are ended with a `;`

### Compiling and Running Are Separate Steps
- compile with `rustc` which is the compiler and comes with `rust`
- to run, just call the executable that's generated, like above


### rust formatting
`rustfmt main.rs` would be how you format the rust file. Probably more to look into here like doing it on save. Function called from command line and comes with rust now.

## 1.3 Hello, Cargo!
Cargo is Rust’s build system and package manager. It reminds me a lot of NPM
```bash
# check install and version
cargo --version

# make a new project
cargo new hello_cargo

# initialize an existing project
mk example
cd example
cago init
```
- this command makes a new cargo project, with a `.toml` file and a src folder, with `main.rs`. it also uses git, which you can configure if you want to, but likely never will.

### the Cargo.toml
- [TOML](https://github.com/toml-lang/toml)(Tom’s Obvious, Minimal Language)
- This is the file

```toml
[package]
name = "hello_cargo"
version = "0.1.0"
authors = ["mostlyfocusedmike <mostlyfocusedmike@gmail.com>"]
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```
- So basically it's just like every other yaml type and it's got a package section with info and dependecies, which is currently empty
- the author is filled from global git values, and the name is what's given
- edition is the version of rust you're using, and currently in 2020 it's still "2018"
> "The last line, [dependencies], is the start of a section for you to list any of your project’s dependencies. In Rust, packages of code are referred to as crates.
- the main.rs file is just the hello world println function

### Building and Running a Cargo Project
build with `build`, run alone with `run`, and make sure your project can compile with `check`. `check` is much faster, so if you don't care about running, but making sure you made no mistake, use `check` periodically (probably a way to do it on save or something)

```bash
# in the root of hello_cargo where .toml is
cargo build
cargo run
cargo check
```
- run can also be used to compile *and* run if you make changes
- Instead of saving the result of the build in the same directory as our code, Cargo stores it in the target/debug directory and also locks in the dependencies in an auto generated `Cargo.lock`
- That is the build for dev, to build the release version with optimizations do

```bash
cargo build --release
```
- which will store the build in the /target/release dir instead of the /target/debug dit
- When you clone down a rust project, like `npm install` first thing you do is `cargo build`
- Add the /target file to your gitignore, you don't want to commit that data to github (it will include secrets since it's your build)

## Running on save 
You need to install cargo-watch
```sh
cargo install cargo-watch # this is a global install
cargo watch -x run # this will run (the default `cargo watch` just runs `check`)
```

# Ch. 2: Programming a Guessing Game
- Use `cargo new` and then insert this into the src main.rs file:

```rs
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
```

## std:io
- We need to import some things from the standard library and `use` is how to import things
- we need to import things because rust doesn't want to just import everything in the entire lang, this is a balance
    - check out what *does* get imported before each program runs, in what is called the [prelude](https://doc.rust-lang.org/stable/std/prelude/index.html)
- also the `::` is associated function, which means it comes from the type, not an instance of the type (think static functions)
- We need the io library to process input and output from the console

## Storing Values with Variables
- in Rust you use `let` to define variables (`const` too)
- by default variables are immutable, but you can set them to be mutable with `mut`

```rs
let foo = 12; // immutable
let mut bar = 43; // mutable
```

- we're just going to talk about importing crates for now, since more mechanics are fully explained in Ch. 3.

## Using a Crate to Get More Functionality
- dependencies go in the dependencies section of the toml file

```toml
[dependencies]
rand = "0.5.5"
```

- the version is semantic, and that's technically short for "^0.5.5"
- To actually load this via the command line instead of manually editing it (which is also fine), you can:

```bash
# install globally
cargo install cargo-edit

# add latest version of package
cargo add rand

# add specific version of package
cargo add rand --vers "0.5.5"

# update your build after adding package
cargo update

# to remove
cargo rm rand
```

- If you don't run the update after installing a package, the next time you build or run, it will auto update
- The amazing thing that Rust does for docs is let you investigate the docs for each package installed (and your whole project really) by doing `cargo doc --open`

## Check the program
- the program has a lot of useful examples, but since everything in it will be covered in more detail later, we'll talk about those parts when we get to them in the book.
- Chapter references have been listed for relevant sections


# Ch.3 Common Programming Concepts

## Variables
- You declare variables with the `let` keyword, they are immutable by default
- You can make them mutable with the `mut` keyword
- Rust can infer types, or you can declare them expllicitly (sometimes you do have to explicitly declare)


```rust
fn main() {
    let x = 5; // immutable
    let mut y = 6; // mutable
    let z: u32 = 9; // explicit type
}
```
- with small structures, it is "Rusty" to copy and return new instances, but for larger structs it might make more sense to mutate in place

### variables vs constants
- constants exist, and are declared with `const` keyword
- they can never be `mut`
- they must be explicity type defined
- they can be declared in any scope, including global (so outside the `main` function)
- They must have a constant expression, no function or computed value (you have to give the literal value)


```rust
const MAX_POINTS: u32 = 100_000;

fn main() {
    println!("Look it's {}", MAX_POINTS);
}
```

### Shadowing Variables
- A variable can't change types, but you can essentially redeclare them to get around this
- common for taking user input as a string and converting to number
```rust
// they type definitions aren't required, just for demonstation
let mut guess: String = String::new();
io::stdin().read_line(&mut guess).expect("Failed to read line");
let guess: u32 = guess.trim().parse().expect("Please type a number!");
```

- Shadowing is important becuase even if a variable is mutable, it can't change it's type

```rust
// This breaks
let mut guess: String = String::new();
io::stdin().read_line(&mut guess).expect("Failed to read line");
guess: u32 = guess.trim().parse().expect("Please type a number!");
```
- Shadowing allows you to keep the same name without having to do things like `guess_num` and `guess_string`
  - That's allowed, just not "Rusty"

------------------------------------------------------------------------
## Basic Data Types
- Rust is a *statically typed* language, which means that all types must be known at compile time
- They can be inferred or explicitly declared
- Sometimes you have to be explicit if a function can return more than one type, like `parse`
  - It can return any number type (we'll get to those in literally one section) so you have to explicitly tell it

```rust
let guess: u32 = "42".parse().expect("Not a number!");
let guess: i32 = "42".parse().expect("Not a number!");
// and many more possibilities
```
- There are many types in Rust, we'll look at 2 subsets: Scalar and Compound. Scalars represents a single value and compound represents types that combine multiple types

- There are 4 main scalar types: integers, floating-point numbers, Booleans, and characters
  - Characters are NOT strings, that's a more complex type

## Integer Types
- Integers are whole numbers and they can be signed (negative or positive) or unsigned (only positives allowed)
- the default type is `i32` which is a signed 32 bit number

```rust
let x = 12 // type is i32
```
- Here are the full list of types

| Length  | Signed | Signed in Decimal                                       | Unsigned | Unsigned in Decimal             |
|---------|--------|---------------------------------------------------------|----------|---------------------------------|
| 8-bit   | i8     | -128 to 127                                             | u8       | 0 to 255                        |
| 16-bit  | i16    | -32,768 to 32,767                                       | u16      | 0 to 65535                      |
| 32-bit  | i32    | -2,147,483,647 to 2,147,483,646                         | u32      | 0 to 4,294,967,294              |
| 64-bit  | i64    | -9,223,372,036,854,776,000 to 9,223,372,036,854,775,999 | u64      | 0 to 18,446,744,073,709,551,999 |
| 128-bit | i128   | big.                                                    | u128     | bigger.                         |
| arch    | isize  | System dependent                                        | usize    | Sytem dependent                 |

- the `arch` one means "rely on system" so if you're on a 32 bit system, it's 32 bits, but if it's a 64 bit system, it's 64
- You aren't limited to decimals in Rust, you can use other number literals:

| Number Literals | Example |
|-----------------|---------|
| Decimal	| 98_222 |
| Hex |	0xff |
| Octal |	0o77 |
| Binary |	0b1111_0000 |
| Byte | (u8 only)	b'A' |

```rust
let x = 0b1111_0000;
println!("{}", x);
// prints 240
```

### Integer overflow
- In development, warnings will fire if you overflow an integer type, but in production releases the variable will just wrap (so 255 would wrap back to 0) unless you use:

> Wrap in all modes with the wrapping_* methods, such as wrapping_add
>
> Return the None value if there is overflow with the checked_* methods
>
> Return the value and a boolean indicating whether there was overflow with the overflowing_* methods
>
> Saturate at the value's minimum or maximum values with saturating_* methods

## Floating Point
- There are 2 kinds `f32` and `f64`
- `f32` is single precision and `f64` is double
- The default is `f64`

```rust
let x = 2.0; // f64
let y: f32 = 3.0; // f32
```


## Numeric Operations
- Pretty standard here. Only gotcha is when using `pow()` you have to explicitly set the type of the base number;
- [Appendix B](https://doc.rust-lang.org/book/appendix-02-operators.html) has every operator (including logical)

```rust
let sum = 5 + 10;
let difference = 95.5 - 4.3;
let product = 4 * 30;
let quotient = 56.7 / 32.2;
let remainder = 43 % 5;

let a: i32 = 5;
println!("{}", a.pow(2)); // 25
```

## Booleans
- Rust uses `true` and `false` and does not coerce truthy and falsy values like JS does
- `bool` is the type name

```rust
let t = true;
let f: bool = false; // with explicit type annotation
```

## The char type
- Chars are individual unicode characters (four bytes big). Note that `String Literal` use `"` and chars use `'`
```rust
let c = 'z';
let z = 'ℤ';
let heart_eyed_cat = '😻';
```

## String Slices / String Literals
- String slices are NOT strings, their type is `str` or `&str`
- They are denoted by `"` as we said above.
- You cannot mutate them, but you can convert to a `String` with `.to_string` or `String::from()`
- MORE ON THIS IN CHAPTERS 4 and 8

```rust

// this is a str
let s = "Hello, world";
println!("{}", s);
// str variables can be mutable, that does not mean the str value can be altered like a String can
let mut s = "Hello, world";
println!("{}", s);
s = "Essentially just a new value"
println!("{}", s);


// This is a String, notice that the value itself is what mutates
// variable must still be mut as well
let s = "Hello".to_string();
println!("{}", s);

s.push_str(", world.");
println!("{}", s);
```

## Compound Types
- Rust has 2 primitive types: tuples and arrays

## Tuples
- Tuples are for a fixed number of values, all of which can have different types
- Types can be explicit or inferred

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);
let tup2 = ("hi", 12);
```

- Access using dot notation with indexes
```rust
let x = (500, 6.4, 1);
let five_hundred = x.0;
let six_point_four = x.1;
let one = x.2;
ll!(five_hundred, six_point_four, one);

// this is standard assignment, so mut works like usual
```

- OR destructuring. The gotcha here is, you have to assign EVERY element, so feel free to use "_" for useless ones

```rust
// Destructuing
let tup = (500, 6.4, 1);
let (_, y, _) = tup;
println!("The value of y is: {}", y);

// if you want mut it goes inside the () per each variable that wants to be mutable
let tup = (500, 6.4, 1);
let (mut x, y, z) = tup;

ll!(x, y, z);
x = 12;
ll!(x, y, z);
```

## Arrays
- Unlike most arrays in other langs, Rust arrays are ALSO fixed length, just like tuples
- the Difference between them is that tuples can store multiple types, while arrays must have all elements be the same type

```rust
// explicit type: [value; length]
let a: [i32; 5] = [1, 2, 3, 4, 5];
```

- To get an array full of the same element, use a semicolon to specify the element, and then the number of times

```rust
let a = [3; 5];
// [3, 3, 3, 3, 3]
```

- Accessing uses brackets instead of dot (dots are only for tuples)

```rust
    let a = [1, 2, 3, 4, 5];
    let two = a[1]


    ll!(two);
```


## Functions 
Rust uses snake case on function and variable names. Rust also has implicit returns if you omit the ";" at the end of the line. You can also explicit return. When returning a value, you need to add `->` and the type that gets returned. Also, when defining parameters, you must explicitly type each one. 

```rs
fn main() {
    another_function(5);
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn five() -> i32 {
    5
}
fn five_too() -> i32 {
    return 5
}
```

> Statements are instructions that perform some action and do not return a value. Expressions evaluate to a resulting value.

You can return an expression from a function. But remember adding the ; at the changes expressions to statements

# Ch 4: Understanding ownership
- There's a unique concept in Rust to handle memory allocation. Instead of using a garbage collector or having the dev decide memory management, Rust uses a system of ownership to handle things, and won't compile if any rules are broken

- Stack is for fixed length, knowable memory allocation at compile time, heap is for unknowable things that can change size (like vectors and Strings)

- The concept of ownership needs more complext data types to work: 

```rust
// fine
fn munch(num: i32) {
  println!("{}", num);
}
fn main() {
  let num1 = 12;
  munch(num1);
  munch(num1);
}

// about to find out why this fails
fn munch(word: String) {
  println!("{}", word);
}
fn main() {
  let word = String::from("hi there");
  munch(word);
  munch(word);
}
```
- Ownership only comes into play when dealing with heap variables. Basically, anything that can be **truly** mutated (not just reassigned like _mut_) deals with ownership

- the three main rules of ownership are: 
  1. Each value in Rust has an owner.
  1. There can only be one owner at a time.
  1. When the owner goes out of scope, the value will be dropped.


```rs
{
  let s = String::from("hello"); // s is valid from this point forward
        // do stuff with s
} 
// this scope is now over, and s is no longer valid
```
- once we move out of the brackets `s` goes out of scope, and so rust calls the `drop` function internally

## Movement and copying 
example: 

```rs
let s1 = String::from("hello");
let s2 = s1;
println!("{}, world!", s1);
```
the value of s1 saves a pointer to the heap for the String value. It then copies this pointer to `s2`, but not the value on the heap. 'This is a problem: when s2 and s1 go out of scope, they will both try to free the same memory.' This is called a double free error and leads to security and corruption concerns.

Because of this, when rust "copies" a pointer in this way, it actually calls it a "move." So in the above example, once you copy s1 to s2, you can no longer reference s1, it is already dropped. 

By deafualt, rust will not clone values on the heap, if you want that, to make 2 truly distinct values, you can use `clone()` or other cloning methods. 

```rs
fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}
```

- stack variables do not need to be cloned, as they are primitives on the stack, so it's easy to truly copy their values around when needed: 

```rs
fn main() {
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);
}
```
You can annotate a type with Copy if you want, however Rust will block you if any part of this type already had the `Drop` trait

Here are some of the built in types with Copy: 

- All the integer types, such as u32.
- The Boolean type, bool, with values true and false.
- All the floating point types, such as f64.
- The character type, char.
- Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.


## Functions and , and returning values and scope 
Functions take ownership, just like variable assignments do. 


```rs 
fn main() {
  let s = String::from("hello");  // s comes into scope

  takes_ownership(s);             // s's value moves into the function...
                                  // ... and so is no longer valid here

  let x = 5;                      // x comes into scope

  makes_copy(x);                  // x would move into the function,
                                  // but i32 is Copy, so it's okay to still
                                  // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
```

```rs 
fn main() {
  let s1 = gives_ownership();         // gives_ownership moves its return
                                      // value into s1

  let s2 = String::from("hello");     // s2 comes into scope

  let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                      // takes_and_gives_back, which also
                                      // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {          // gives_ownership will move its
                                          // return value into the function
                                          // that calls it

  let some_string = String::from("yours"); // some_string comes into scope

  some_string                             // some_string is returned and
                                          // moves out to the calling
                                          // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}
```
- all this transfer of ownership can be tedious. Sometimes, you just want to temporarily borrow something 

## References and borrowing
This is valid but annoying 

```rs
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
```
We just want to read the value, not much. 

Instead we can use a `reference` to the String.
> A reference is like a pointer in that it’s an address we can follow to access the data stored at that address; that data is owned by some other variable. Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type for the life of that reference.

Basically if you just want to read, use a reference. The way to use and the way you annotate Type is with `&`:

```rs 
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```
With references you can use them as many times as you want. 

But you *can* mutate a borrowed reference if you *really* want to: 

## Mutable References
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}


> Mutable references have one big restriction: if you have a mutable reference to a value, you can have no other references to that value. This code that attempts to create two mutable references to s will fail:

```rs 
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s;

println!("{}, {}", r1, r2);
```

Blocking this behavrior prevents race conditions.

You also can't use both immutable and mutable references together in the same scope:

```rs 
 let mut s = String::from("hello");

let r1 = &s; // no problem
let r2 = &s; // no problem
let r3 = &mut s; // BIG PROBLEM

println!("{}, {}, and {}", r1, r2, r3);
```
In order to get the mutable reference functionality back, you have to drop the 

```rs
fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}

```
## Dangling References 

Rust blocks dangling references (a pointer that points at something on the heap that's already been dropped or given to something else)

```rs
fn main() {
    let reference_to_nothing = dangle(); // BAD BAD
}

fn dangle() -> &String {
    let s = String::from("hello");
    &s  
}
```

## Another book
https://www.cs.brandeis.edu/~cs146a/rust/doc-02-21-2015/book/README.html
I don't really know what that book is or who wrote it but it's great