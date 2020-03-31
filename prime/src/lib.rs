pub fn is_prime(number: i64) -> bool {
    let border = ((number as f64).sqrt() + 1.0) as i64;

    for i in 2..border {
        if number % i == 0 {
            return false;
        }
    }
    return true;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_prime() {
        let prime = 13;
        let not_prime = 14;

        assert_eq!(true, is_prime(prime));
        assert_eq!(false, is_prime(not_prime));
    }
}
