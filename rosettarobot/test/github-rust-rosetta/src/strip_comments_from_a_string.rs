// http://rosettacode.org/wiki/Strip_comments_from_a_string

fn strip_comments(str: &str) -> &str {
    let markers = ['#', ';'];
    str.find(markers.as_slice()).map_or(str, |i| str.slice_to(i)).trim()
}

#[test]
fn test_strip_comments() {
    let inputs = ["apples, pears # and bananas",
                  "apples, pears ; and bananas",
                  "  apples, pears "];
    let output = "apples, pears";

    for &input in inputs.iter() {
        assert_eq!(strip_comments(input), output)
    }
}

#[cfg(not(test))]
fn main() {
    let inputs = ["apples, pears # and bananas",
                  "apples, pears ; and bananas",
                  "  apples, pears "];

    for &input in inputs.iter() {
        println!("Input: {}\nStripped: {}", input, strip_comments(input))
    }
}
