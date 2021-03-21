// creating a trait with no default behavior
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// adding (and defining) the trait (note that the signature must match)
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// And here it is in use:
use chapter10::{self, Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}

// this would be a trait with a default behavior that can be overidden
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

// Traits can define more than one function, and default implementations can reference other trait functions
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

// now for Tweet, we only need to define one method, the default summarize will work fine
impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// You can use traits as parameters and in generics

// as type in a function using `impl Trait syntax`
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
// that will will expect the type to have the Summary trait
// fun fact: thats syntactic sugar for `Trait Bound Syntax`
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// you can specify more than one trait with +
pub fn notify(item: &(impl Summary + Display)) { }

// expanded that would be:
pub fn notify<T: Summary + Display>(item: &T) { }

// it can get wordy to define multiple, you can use a `where` clause to neaten it up
fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{


// returning a trait works like how you think
fn returns_summarizable() -> impl Summary { }

// as parameter in generic