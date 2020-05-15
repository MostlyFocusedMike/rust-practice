# Rust Notes
Here are my rust notes. Primarily based off the Book of Rust
# Ch.1 Getting Started

# 1.1 Installation
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


# 1.2 Hello Rust!
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

## Anatomy of a Rust Program
- functions look like the above, but rust uses a special "main" funciton, it takes nothing and returns nothing. Needs to be top level (maybe)
- `println!` is not just a function, but a macro, you can tell because it uses the `!`. Regular functions don't have that
- Most lines in rust are ended with a `;`

## Compiling and Running Are Separate Steps
- compile with `rustc` which is the compiler and comes with `rust`
- to run, just call the executable that's generated, like above


## rust formatting
`rustfmt main.rs` would be how you format the rust file. Probably more to look into here like doing it on save. Function called from command line and comes with rust now.

# 1.3 Hello, Cargo!
Cargo is Rust’s build system and package manager. It reminds me a lot of NPM
```bash
# check install and version
cargo --version

# make a new file
cargo new hello_cargo
```
- this command makes a new cargo project, with a `.toml` file and a src folder, with `main.rs`. it also uses git, which you can configure if you want to, but likely never will.

## the Cargo.toml
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

## Building and Running a Cargo Project
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