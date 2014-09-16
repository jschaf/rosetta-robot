// Implements http://rosettacode.org/wiki/Happy_numbers

use std::collections::treemap::TreeSet;
#[cfg(not(test))]
use std::iter::count;

fn digits(mut n: uint) -> Vec<uint> {
    let mut ds = vec![];
    if n == 0 {
        return vec![0];
    }
    while n > 0 {
        ds.push(n % 10);
        n /= 10;
    }
    ds.reverse();
    ds
}

fn is_happy(mut x: uint) -> bool {
    let mut past = TreeSet::new();
    while x != 1 {
        // Take the sum of the squares of the digits of x
        x = digits(x).iter().fold(0, |a, &b| a + b * b);

        // The number is not happy if there is an endless loop
        if past.contains(&x) {
            return false
        }

        past.insert(x);
    }
    true
}

#[cfg(not(test))]
fn main() {
    // Print the first 8 happy numbers
    let v: Vec<uint> = count(1u, 1)
        .filter(|x| is_happy(*x))
        .take(8)
        .collect();
    println!("{}", v)
}

#[test]
fn test_digits() {
    assert_eq!(digits(0), vec![0]);
    assert_eq!(digits(1), vec![1]);
    assert_eq!(digits(2), vec![2]);
    assert_eq!(digits(10), vec![1, 0]);
    assert_eq!(digits(11), vec![1, 1]);
    assert_eq!(digits(101), vec![1, 0, 1]);
    assert_eq!(digits(1000), vec![1, 0, 0, 0]);
}

#[test]
fn test_is_happy() {
    let happys = [1u, 7, 10, 13, 19, 23, 28, 31, 1607, 1663];
    let unhappys = [0u, 2, 3, 4, 5, 6, 8, 9, 29, 1662];

    assert!(happys.iter().all(|&n| is_happy(n)));
    assert!(unhappys.iter().all(|&n| !is_happy(n)));
}
