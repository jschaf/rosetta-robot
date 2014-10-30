// http://rosettacode.org/wiki/FASTA_format
// Fasta reader in Rust 0.11-pre
// Ported and adapted from rosettacode D example

use std::io::fs::File;
use std::io::BufferedReader;

// We use a type parameter bound `<T: Buffer>` to accept all kinds of buffers
fn format_fasta<T: Buffer>(reader: &mut T) -> String {
    reader.lines().map(|l| l.unwrap()).fold(String::new(), |mut out, line| {
        // We need to trim new lines
        let ln = line.as_slice().trim();

        // Lines that begin with '>' require special treatment
        match ln.slice_to(1) {
            ">" => {
                if out.len() > 0 {
                    out.push('\n');
                }

                // Push skipping the '>'
                out.push_str(ln.slice_from(1));
                out.push_str(": ");
            }
            // Other lines are just pushed
            _ => out.push_str(ln)
        }
        out
    })
}

fn read_file() -> String {
    let file = File::open(&Path::new("src/resources/test_data.fasta"));
    format_fasta(&mut BufferedReader::new(file))
}

#[cfg(not(test))]
fn main() {
    let s = read_file();
    println!("{}", s);
}

#[test]
fn test_format_fasta() {
    let s = read_file();
    assert_eq!(s.as_slice(), "Rosetta_Example_1: THERECANBENOSPACE
Rosetta_Example_2: THERECANBESEVERALLINESBUTTHEYALLMUSTBECONCATENATED");
}
