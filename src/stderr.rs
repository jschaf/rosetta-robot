// Implements http://rosettacode.org/wiki/Hello_world/Standard_error
// not_tested
#![allow(unused_must_use)]

use std::io;

fn main() {
    let mut stderr = io::stderr();
    stderr.write(b"Goodbye, World!\n");
}
