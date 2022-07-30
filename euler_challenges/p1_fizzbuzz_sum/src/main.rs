// If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
// Find the sum of all the multiples of 3 or 5 below 1000.
// https://projecteuler.net/problem=1

pub fn sum_under(limit: i32) -> i32 {
    /*  Non flashy way */
    // let numbers: Vec<i32> = (1..limit).collect();
    // let mut total = 0;
    // for i in numbers.iter() {
    //     if i % 3 == 0 || i % 5 == 0 {
    //         total += i;
    //     }
    // }
    // return total;
    
    let numbers: Vec<i32> = (1..limit).collect();
    return numbers.iter().filter(|&n| *n % 3 == 0 || *n % 5 == 0).sum();
}
        
fn main() {
    println!("{}", sum_under(10));
    println!("{}", sum_under(1000));
}   


#[cfg(test)]
mod tests {
    use super::sum_under;

    #[test]
    fn it_works() {
        let assumed_answer = 233168;
        assert_eq!(23, sum_under(10));
        assert_eq!(assumed_answer, sum_under(1000));
    }
}