//Implements http://rosettacode.org/wiki/Sorting_algorithms/Bubble_sort
use std::iter::range_inclusive;

/// Progress through the slice and 'bubble' elements up until they are in order.
fn bubble_sort<T: PartialOrd>(v: &mut [T]) {
    range_inclusive(1, v.len()).rev().all(|length| {
        let mut changes = 0u;

        for index in range(0, length - 1) {
            if v[index] > v[index + 1] {
                changes += 1;
                v.swap(index, index + 1);
            }
        }

        // Continue to iterate if any 'bubble-ing' took place
        changes > 0
    });
}

#[cfg(not(test))]
fn main() {
    let mut numbers = [4i, 65, 2, -31, 0, 99, 2, 83, 782, 1];
    bubble_sort(numbers);
}

#[cfg(test)]
mod test {
    fn check_sort<T: PartialOrd>(v: &mut [T]) {
        super::bubble_sort(v);

        for i in range(1u, v.len()) {
            assert!(v[i - 1] <= v[i]);
        }
    }

    #[test]
    fn rosetta_vector() {
        let mut numbers = [4i, 65, 2, -31, 0, 99, 2, 83, 782, 1];
        check_sort(numbers);
    }

    #[test]
    fn empty_vector() {
        let mut numbers: Vec<int> = Vec::new();
        check_sort(numbers.as_mut_slice());
    }

    #[test]
    fn one_element_vector() {
        let mut numbers = [0i];
        check_sort(numbers);
    }

    #[test]
    fn repeat_vector() {
        let mut numbers = [1i, 1, 1, 1, 1];
        check_sort(numbers);
    }

    #[test]
    fn worst_case_vector() {
        let mut numbers = [20i, 10, 0, -1, -5];
        check_sort(numbers);
    }

    #[test]
    fn already_sorted_vector() {
        let mut numbers = [-1i, 0, 3, 6, 99];
        check_sort(numbers);
    }
}
