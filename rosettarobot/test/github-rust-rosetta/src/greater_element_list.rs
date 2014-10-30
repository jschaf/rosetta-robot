// http://rosettacode.org/wiki/Greatest_element_of_a_list

use std::fmt::Show;

fn main() {
    find_max("first", [1i, 2, 3, 4, 5, 6, 7, 8, 9]);
    find_max("second", [123i, 3543, 23, 432, 5, 2, 34, 234, 234,
                        2, 4, 234, 23, 4, 24, 25, 7, 658, 68]);
    find_max("third", ['a', 'b', 'c', 'd', 'e']);
    find_max("fourth", ["Bonjour", "Hola", "Hello", "Hallo", "Buongiorno"]);
}

fn find_max<T: Show + Ord>(count: &str, list: &[T]) {
    let max = list.iter().max_by(|&x| x).unwrap();
    println!("Max of the {} list: {}", count, max);
}
