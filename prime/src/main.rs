extern crate prime;
use std::fs;
fn main() {
    let number: i64 = 13;
    if prime::is_prime(number) {
        println!("is prime")
    } else {
        println!("is not prime")
    }

    let data = fs::read_to_string("test.txt").expect("something is wrong");
    let data = data.split("\n");
    for num in data {
        //let n = num.parse::<i64>();
        match num.parse::<i64>() {
            Ok(n) => {
                if prime::is_prime(n) {
                    println!("is prime")
                } else {
                    println!("is not prime")
                }
            }
            _ => println!("finish"),
        }
    }
}
