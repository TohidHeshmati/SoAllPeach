use std::fs;
use std::*;

fn main() {
    let args:Vec<String> = env::args().collect();
    let data = fs::read_to_string(&args[1]).expect("something is wrong");
    let data = data.split("\n");

    for num in data {
        match num.parse::<i64>() {
            Ok(n) => {
                if is_prime(n) {
                    println!("1")
                } else {
                    println!("0")
                }
            }
            _ => continue,
        }
    }
}

fn is_prime(number: i64) -> bool {
    if number <=3 {
        return true;
    } else if number % 2 == 0 {
        return false;
    }
    let border = ((number as f64).sqrt() + 1.0) as i64;

    for i in (3..border).step_by(2) {
        if number % i == 0 {
            return false;
        }
    }
    return true;
}
