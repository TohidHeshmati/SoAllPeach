extern crate prime;
use std::fs;
use std::*;

fn main() {
    let args:Vec<String> = env::args().collect();
    let data = fs::read_to_string(&args[1]).expect("something is wrong");
    let data = data.split("\n");

    for num in data {
        match num.parse::<i64>() {
            Ok(n) => {
                if prime::is_prime(n) {
                    println!("1")
                } else {
                    println!("0")
                }
            }
            _ => continue,
        }
    }
}
