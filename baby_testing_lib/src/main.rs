// use std::error::Error;
use std::fmt::Display;
//
#[allow(dead_code)]
fn it<F: Fn() -> Result<bool, String>>(test_name: &str, f: F) {
    match f() {
        Ok(_val) => println!("\tIt {} ... OK", test_name),
        Err(_err) => {
            println!("\t{} ... FAIL", test_name);
        },
    };

}
#[allow(dead_code)]
fn describe(suite_name: &str) {
    println!("\n{} Tests running ...", suite_name);
}

#[allow(dead_code)]
fn is_equal<T: Display + PartialOrd>(val1: T, val2: T) -> Result<bool, String> {
    let test = val1 == val2;
    match test {
        true => Ok(true),
        false => Err(String::from("are not equal")),
    }
}

#[allow(dead_code)]
fn is_true(val1: bool) -> bool {
    val1 == true
}

#[test]
fn equality_tests() {
    describe("Equality");
    it("Checks equality", || {
        is_equal(4,3)
    });

    it("Shyould work this time", || {
        println!("HELLO");
        is_equal(3,3)
    });

    it("still works", || {
        is_equal(3,3)
    });
}



fn main() {
    println!("Hello, world!");
}
