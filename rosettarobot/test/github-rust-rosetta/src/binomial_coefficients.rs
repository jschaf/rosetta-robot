// http://rosettacode.org/wiki/Evaluate_binomial_coefficients

extern crate num;
extern crate core;
use num::bigint::BigUint;
use core::num::One;

fn binomial(n: uint, mut k: uint) -> BigUint {
    // Since binomial(n, k) = binomial(n, n - k), we might as well use
    // the smaller k to optimize
    if n - k < k {
        k = n - k;
    }

    // Compute the coefficient
    let mut res: BigUint = One::one();
    for i in range(1, k + 1) {
        res = res * FromPrimitive::from_uint(n - k + i).unwrap();
        res = res / FromPrimitive::from_uint(i).unwrap();
    }

    res
}

#[cfg(not(test))]
fn main() {
    println!("{}", binomial(5, 3));
}

#[test]
fn test_binomial() {
    use std::from_str::FromStr;

    assert_eq!(binomial(20, 0), binomial(20, 20));
    assert_eq!(binomial(20, 15), binomial(19, 14) + binomial(19, 15));
    assert_eq!(binomial(5, 3), FromPrimitive::from_uint(10).unwrap());
    assert_eq!(binomial(31, 17), FromPrimitive::from_uint(265182525).unwrap());
    assert_eq!(binomial(300, 30),
        FromStr::from_str("173193226149263513034110205899732811401360").unwrap());
}

