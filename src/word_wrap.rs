// http://rosettacode.org/wiki/Word_wrap

// Using the minimum length greedy algorithm
// http://en.wikipedia.org/wiki/Word_wrap#Minimum_length

// Implemented as a lazy String iterator, returning a wrapped line each time

use std::str::Words;
use std::mem::swap;

pub struct WordWrap<'a> {
    words: Words<'a>,
    line_length: uint,
    next_line: String
}

impl<'a> WordWrap<'a> {
    fn new(text: &'a str, line_length: uint) -> WordWrap {
        WordWrap {
            words : text.words(),
            line_length : line_length,
            next_line: String::new(),
        }
    }
}

impl<'a> Iterator<String> for WordWrap<'a> {
    fn next(&mut self) -> Option<String> {
        // Move anything left over from last run to this_line
        let mut this_line = String::new();
        swap(&mut self.next_line, &mut this_line);

        let mut space_left = self.line_length - this_line.as_slice().char_len();
        const SPACE_WIDTH: uint = 1;

        // Loop, adding words until we run out of words or hit the line length
        for word in self.words {
            let word_length = word.char_len();

            // If not the first word for this line
            if space_left != self.line_length {
                if word_length + SPACE_WIDTH > space_left {
                    // Out of space, save word for next line
                    self.next_line.push_str(word);
                    break;
                }
                else {
                    // Add a space and keep going
                    this_line.push(' ');
                    space_left -= SPACE_WIDTH;
                }
            }

            // Add word to this line
            this_line.push_str(word);
            space_left -= word_length;
        }

        if this_line.is_empty() { None } else { Some(this_line) }
    }
}

#[cfg(not(test))]
fn main () {
    let text =
        "In olden times when wishing still helped one, there lived a king \
         whose daughters were all beautiful, but the youngest was so beautiful \
         that the sun itself, which has seen so much, was astonished whenever \
         it shone in her face.  Close by the king's castle lay a great dark \
         forest, and under an old lime tree in the forest was a well, and when \
         the day was very warm, the king's child went out into the forest and \
         sat down by the side of the cool fountain, and when she was bored she \
         took a golden ball, and threw it up on high and caught it, and this \
         ball was her favorite plaything.";

    for &length in [72u, 80u].iter() {
        println!("Text wrapped at {}", length);
        for line in WordWrap::new(text, length) {
            println!("{}", line);
        }
        println!("");
    }

}

#[test]
fn test_empty_string() {
    assert_eq!(WordWrap::new("", 80).next(), None);
}

#[test]
fn test_single_word_shorter_than_line() {
    assert_eq!(WordWrap::new("Hello", 80).next().unwrap().as_slice(), "Hello");
}

#[test]
fn test_two_words_shorter_than_line() {
    assert_eq!(WordWrap::new("Hello world", 80).next().unwrap().as_slice(),
               "Hello world");
}

#[test]
fn test_wrap_second_word() {
    let mut w = WordWrap::new("Hello world", 10);
    assert_eq!(w.next().unwrap().as_slice(), "Hello");
    assert_eq!(w.next().unwrap().as_slice(), "world");
}

#[test]
fn test_wrap_punctuation() {
    let mut w = WordWrap::new("Hello, world", 6);
    assert_eq!(w.next().unwrap().as_slice(), "Hello,");
    assert_eq!(w.next().unwrap().as_slice(), "world");
}

#[test]
fn test_squash_multiple_spaces() {
    let mut w = WordWrap::new(" Hello  to the    world    ", 10);
    assert_eq!(w.next().unwrap().as_slice(), "Hello to");
    assert_eq!(w.next().unwrap().as_slice(), "the world");
    assert_eq!(w.next(), None);
}

#[test]
fn test_unicode() {
    let mut w =
        WordWrap::new("Nous étions à l'Étude, quand le Proviseur entra", 11);
    assert_eq!(w.next().unwrap().as_slice(), "Nous étions");
    assert_eq!(w.next().unwrap().as_slice(), "à l'Étude,");
    assert_eq!(w.next().unwrap().as_slice(), "quand le");
    assert_eq!(w.next().unwrap().as_slice(), "Proviseur");
    assert_eq!(w.next().unwrap().as_slice(), "entra");
}
