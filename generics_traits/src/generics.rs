// This is how to do generics, it's mad though becuase there is no guarantee that the given type uses > comparison
// You'd need to use a generic that accepts things with traits
fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest { // a problem
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

// This is how you do generic structs
struct Point1<T> {
    x: T,
    y: T,
}

fn main2() {
    let both_integer = Point1 { x: 5, y: 10 };
    let both_float = Point1 { x: 1.0, y: 4.0 };
    let integer_and_float = Point1 { x: 5, y: 4.0 };
}

// you can have more than one type
struct Point2<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point2<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main3() {
    let p = Point2 { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}


// This is enums (this is the how the std Option looks)
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

// fun fact: no performance hit at compile time because of Monomorphization