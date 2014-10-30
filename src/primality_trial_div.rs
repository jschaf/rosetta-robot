//Implements http://rosettacode.org/wiki/Primality_by_Trial_Division

use std::iter::range_step;

fn is_prime(number: int) -> bool {
    if number % 2 == 0 && number != 2 {
        return false;
    }

    let limit = (number as f32).sqrt() as int + 1;

    // We test if the number is divisible by any odd number up to the limit
    range_step(3, limit, 2).all(|x| number % x != 0)
}

#[cfg(not(test))]
fn main() {
    println!("{:b}", is_prime(15485863)); // The 1 000 000th prime.
    println!("{:b}", is_prime(62773913)); // The product of the 1000th and 1001st primes.
}

#[test]
fn test_one() {
    assert!(is_prime(1));
}

#[test]
fn test_two() {
    assert!(is_prime(2));
}

#[test]
fn test_many() {
    let primes = [3, 5, 7, 11, 13, 17, 19, 23, 29, 31];
    assert!(primes.iter().all(|&x| is_prime(x)));
}
