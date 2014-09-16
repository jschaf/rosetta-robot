// Implements http://rosettacode.org/wiki/Parallel_calculations
// See http://static.rust-lang.org/doc/master/guide-tasks.html for information
// about tasks, channels, future, etc.

#[cfg(test)]
extern crate test;
extern crate sync;

use std::sync::Future;
use prime_decomposition::factor;

mod prime_decomposition;

// Returns the minimal prime factor of a number
fn min_factor(x: uint) -> uint {
    // factor returns a sorted vector, so we just take the first element
    factor(x)[0]
}

// Returns the largest minimal factor of the numbers in a slice
// The function is implemented using a channel
#[cfg(test)]
fn largest_min_factor_chan(numbers: &[uint]) -> uint {
    let (sender, receiver) = channel();

    // Send all the minimal factors
    for &x in numbers.iter() {
        let child_sender = sender.clone();
        spawn(proc() { child_sender.send(min_factor(x)) });
    }

    // Receive them and keep the largest one
    numbers.iter().fold(0u, |max, _| {
        std::cmp::max(receiver.recv(), max)
    })
}

// Returns the largest minimal factor of the numbers in a slice
// The function is implemented using the Future struct
fn largest_min_factor_fut(numbers: &[uint]) -> uint {
    // We will save the future values of the minimal factor in the results vec
    let mut results = Vec::from_fn(numbers.len(), |i| {
        let number = numbers[i];
        Future::spawn(proc() { min_factor(number) })
    });

    // Get the largest minimal factor of all results
    results.mut_iter().map(|r| r.get()).max().unwrap()
}

#[cfg(not(test))]
fn main() {
    // Numbers to be factorized
    let numbers = &[1122725u,
                   1125827,
                   1122725,
                   1152800,
                   1157978,
                   1099726];

    let max = largest_min_factor_fut(numbers);
    println!("The largest minimal factor is {}", max);
}

// We dont have benchmarks because the Bencher doesn't work good with tasks
#[test]
fn test_basic() {
    let numbers = &[25, 80, 256, 55, 18, 19];
    assert_eq!(largest_min_factor_fut(numbers), 19);
}

#[test]
fn test_equivalence() {
    let numbers = &[1122725u,
                   1125827,
                   1122725,
                   1152800,
                   1157978,
                   1099726];
    assert_eq!(largest_min_factor_chan(numbers),
                largest_min_factor_fut(numbers));
}
