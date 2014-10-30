// Implements http://rosettacode.org/wiki/Benford%27s_law
//
// Contributed by Gavin Baker <gavinb@antonym.org>
//

use std::io::{File, BufferedReader};

// Calculate the expected frequency of a digit according to Benford's Law
fn benford_freq(d: u64) -> f32 {
    assert!(d >= 1 && d <= 9);

    (1.0 + 1.0/(d as f32)).log10()
}

// Returns the leading digit of any number
fn first_digit_of(n: u64) -> uint {
    let mut d = n;
    while d > 9 {
        d = d / 10;
    }
    d as uint
}

// Count frequency table using the first digit of each number in a vector
fn benford_distrib(numbers: &Vec<u64>) -> Vec<f32> {

    // Counts

    let mut counts = Vec::<u64>::from_elem(10, 0);

    for num in numbers.iter() {
        let first = first_digit_of(*num);
        *counts.get_mut(first) += 1;
    }

    // Frequencies

    let mut freq = Vec::<f32>::from_elem(10, 0.0);

    for digit in range(1, 10) {
        *freq.get_mut(digit) = counts[digit] as f32 / numbers.len() as f32;
    }

    freq
}

const N: uint = 100;

fn main() {

    // Calculate expected frequencies of all digits according to Benford's Law

    let mut expected_distrib = Vec::<f32>::from_elem(10, 0.0);
    for digit in range(1, 10) {
        *expected_distrib.get_mut(digit) = benford_freq(digit as u64);
    }

    // Load data from the Fibonacci sequence

    let filename = "../src/resources/fib1000.txt";
    let path = Path::new(filename);
    let mut file = BufferedReader::new(File::open(&path));

    let fibs: Vec<u64> = file.lines().map(|x| {
        let s = x.unwrap();
        let n = from_str(s.as_slice().slice(0, 1));
        n.unwrap()
    }).collect();

    // Calculate freuencies of first digits in test data

    let found_distrib = benford_distrib(&fibs);

    // Print the stats to compare actual vs expected

    println!("\nBenford's Law - Digit Distribution");
    println!("\nFirst 1000 Numbers in the Fibonacci Sequence\n");
    println!("digit    expect     found     delta");
    for digit in range(1, 10) {
        let expected_pc = expected_distrib[digit] * 100.0;
        let found_pc = found_distrib[digit] * 100.0;
        let delta_pc = expected_pc - found_pc;

        println!("{}        {:>4.1f}%      {:>4.1f}%    {:>5.2f}%", digit, expected_pc, found_pc, delta_pc);
    }
}
