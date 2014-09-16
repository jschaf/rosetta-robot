// http://rosettacode.org/wiki/Pythagorean_triples
use std::collections::DList;
use std::num::pow;

/// Count the number of Pythagorean triples whose sum are below the specified limit (inclusive).
/// Does a BFS over the tree of primitive Pythagorean triples (see [0]), and uses the fact that
/// each child has a bigger sum than its parent.
/// [0]: http://en.wikipedia.org/wiki/Tree_of_Pythagorean_triples
fn count_pythagorean_triples(below: u64) -> (u64, u64) {
    let mut tot_cnt = 0;
    let mut prim_cnt = 0;
    let mut queue = DList::new();

    // Initiate the BFS with the root of the tree: (3, 4, 5)
    queue.push((3i64, 4i64, 5i64));

    loop {
        match queue.pop() {
            Some((a, b, c)) => {
                // We found a new primitive Pythagorean triplet: (a, b, c).
                // (k*a, k*b, k*c) is a (not necessarily primitive) Pythagorean triplet
                // for any positive integer k.
                // We're interested in those with k*a + k*b + k*c <= below,
                // and the number of them are exactly below / (a + b + c)
                let cur = below / (a + b + c) as u64;
                if cur > 0 {
                    tot_cnt += cur;
                    prim_cnt += 1;

                    // Explore the children of the current node
                    queue.push(( a - 2*b + 2*c,  2*a - b + 2*c,  2*a - 2*b + 3*c));
                    queue.push(( a + 2*b + 2*c,  2*a + b + 2*c,  2*a + 2*b + 3*c));
                    queue.push((-a + 2*b + 2*c, -2*a + b + 2*c, -2*a + 2*b + 3*c));
                }
            },
            None => {
                // We're done, no more nodes to search
                break;
            }
        }
    }

    (tot_cnt, prim_cnt)
}

#[cfg(not(test))]
fn main() {
    for n in range(1, 9) {
        let (tot, prim) = count_pythagorean_triples(pow(10, n));
        println!("Up to 10^{}: {:>10u} triples {:>10u} primitives",
                 n, tot, prim);
    }
}

#[test]
fn test_count_pythagorean_triples() {
    assert_eq!(count_pythagorean_triples(pow(10, 6)), (808950, 70229));
}

