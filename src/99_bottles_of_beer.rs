// Implements http://rosettacode.org/wiki/99_Bottles_of_Beer
use std::string::String;

#[cfg(not(test))]
fn main() {
	for num_bottles in std::iter::range_inclusive(1u, 99).rev() {
        println!("{}", bottles_line(num_bottles, true));
		println!("{}", bottles_line(num_bottles, false));
		println!("Take one down, pass it around...");
		println!("{}", bottles_line(num_bottles - 1, true));
		println!("-----------------------------------");
	}
}

fn bottles_line(num_bottles: uint, on_the_wall: bool) -> String {
    let tail = match on_the_wall {
        true => "of beer on the wall!\n",
        false => "of beer\n"
    };

    match num_bottles {
        0 => format!("No bottles {}", tail),
        1 => format!("One bottle {}", tail),
        n => format!("{} bottles {}", n, tail)
    }
}

#[test]
fn gen_bottle_line() {
    let ln = bottles_line(42, false);
    let ln2 = bottles_line(42, true);

    assert_eq!(ln.as_slice(), "42 bottles of beer\n");
    assert_eq!(ln2.as_slice(), "42 bottles of beer on the wall!\n");
}
