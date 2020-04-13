use std::fs;
use std::*;

fn main() {
    let args:Vec<String> = env::args().collect();
    let data = fs::read_to_string(&args[1]).expect("something is wrong");
    let data = data.split("\n");

    for num in data {
        match num.parse::<i64>() {
            Ok(n) => print_prime(n),
            _ => continue,
        }
    }
}

fn print_prime(number: i64) {
    if number <=3 {
        println!("1")
    } else if number % 2 == 0 || number % 3 == 0 {
        println!("0")
    }
    let border = ((number as f64).sqrt() + 1.0) as i64;

    for i in (5..border).step_by(6) {
        if number % i == 0 || number % (i + 2) == 0 {
            println!("0")
        }
    }
    println!("1")
}