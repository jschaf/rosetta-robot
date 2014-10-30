// Implement data structures for a Huffman encoding tree:
//   http://rosettacode.org/wiki/Huffman_coding

extern crate core;
use std::collections::HashMap;
use std::collections::hashmap::{Occupied, Vacant};
use std::collections::priority_queue::PriorityQueue;

// Each HNode has a weight, representing the sum of the frequencies for all its
// children. It is either a leaf (containing a character), or a HTree
// (containing two children)
struct HNode {
    weight: uint,
    item: HTreeOrHLeaf,
}

enum HTreeOrHLeaf {
    HTree(HTreeData),
    HLeaf(char),
}

struct HTreeData {
    left: Box<HNode>,
    right: Box<HNode>,
}

// Implementing comparison traits (Ord and all its dependencies) such that
// the HNode with the greatest weight is the smallest in a comparison. Basically
// reversing all the comparison operators.
impl Ord for HNode {
    fn cmp(&self, other: &HNode) -> Ordering {
        match self.weight.cmp(&other.weight) {
            Less    => Greater,
            Equal   => Equal,
            Greater => Less,
        }
    }
}

impl PartialOrd for HNode {
    fn partial_cmp(&self, other: &HNode) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for HNode {}
impl PartialEq for HNode {
    fn eq(&self, other: &HNode) -> bool {
        self.weight == other.weight
    }
}

// Takes a non-empty string (function will fail if string is empty) and computes
// the Huffman encoding tree for that string.
fn huffman_tree(input: &str) -> HNode {
    // 1. Loop through all the characters in that string, adding them to a HashMap
    //    of character to frequency.
    let mut freq = HashMap::new();
    for ch in input.chars() {
        match freq.entry(ch) {
            Vacant(entry) => { entry.set(1u); },
            Occupied(mut entry) => { *entry.get_mut() += 1; },
        };
    }

    // 2. For each (character, frequency) pair in the HashMap, add a Leaf to a
    //    PriorityQueue
    let mut queue = PriorityQueue::<HNode>::new();
    for (ch, freq) in freq.iter() {
        let new_node = HNode{
            weight: *freq,
            item: HLeaf(*ch),
        };
        queue.push(new_node);
    }

    // 3. Pop two items with the least weight from the queue, combine them into
    //    a tree as children. The parent node's weight is the sum of the
    //    children's weight. Continue until one item is left on the queue, and
    //    return that item.
    while queue.len() > 1 {
        let item1 = queue.pop().unwrap();
        let item2 = queue.pop().unwrap();
        let new_node = HNode {
            weight: item1.weight + item2.weight,
            item: HTree(HTreeData{
                left: box item1,
                right: box item2,
            }),
        };
        queue.push(new_node);
    }
    queue.pop().unwrap()
}

// Takes a Huffman Tree, traverse it and build a table with each character and
// its encoding string.
fn build_encoding_table(tree: &HNode,
                      table: &mut HashMap<char,String>,
                      start_str: &str) {
    match tree.item {
        HTree(ref data) => {
            build_encoding_table(&*data.left, table,
                               format!("{}0", start_str).as_slice());
            build_encoding_table(&*data.right, table,
                               format!("{}1", start_str).as_slice());
        },
        HLeaf(ch)   => {table.insert(ch, start_str.to_string());}
    };
}

// Attempts to construct a tree, and test that the construction is successful
//    7
//   ----
//  /    \
// 4:'4'  3
//      -----
//     /     \
//    2:'2'  1:'1'
#[test]
fn test_tree_construction() {
    let to_encode = "4444221";
    let tree = huffman_tree(to_encode);
    assert!(tree.weight == 7);
    let children = match tree.item {
        HTree(data) => data,
        HLeaf(_)    => fail!("Tree Missing Children!"),
    };
    let left  = &children.left;
    let right = &children.right;
    assert!(right.weight == 4);
    assert!(left.weight == 3);
    let right_char = match right.item {
        HTree(_)  => fail!("Node is not Leaf Node!"),
        HLeaf(ch) => ch,
    };
    assert!(right_char == '4');
    let children = match left.item {
        HTree(ref data) => data,
        HLeaf(_)    => fail!("Tree Missing Children!"),
    };
    let left  = &children.left;
    let right = &children.right;
    let left_char = match left.item {
        HTree(_)  => fail!("Node is not Leaf Node!"),
        HLeaf(ch) => ch,
    };
    let right_char = match right.item {
        HTree(_)  => fail!("Node is not Leaf Node!"),
        HLeaf(ch) => ch,
    };
    match (left.weight, right.weight) {
        (1, 2) => {
            assert!(left_char == '1');
            assert!(right_char == '2');
        },
        (2, 1) => {
            assert!(left_char == '2');
            assert!(right_char == '1');
        },
        (_, _) => {
            fail!("Incorrect Leaf Nodes");
        }
    };
}

#[test]
// Constructs a table:
//  '4': 1
//  '2': 01 OR 00
//  '1': 00    01
// And tests that the table was correctly constructed
fn test_table_construction() {
    let to_encode = "4444221";
    let tree = huffman_tree(to_encode);
    let mut table = HashMap::<char,String>::new();
    build_encoding_table(&tree, &mut table, "");
    let one  = table['1'].as_slice();
    let two  = table['2'].as_slice();
    let four = table['4'].as_slice();
    assert!(four == "1");
    assert!((one == "01" && two == "00") ||
            (one == "00" && two == "01"));
}

#[cfg(not(test))]
fn main() {
    let to_encode = "this is an example for huffman encoding";
    let tree = huffman_tree(to_encode);
    let mut table = HashMap::<char,String>::new();
    build_encoding_table(&tree, &mut table, "");

    for (ch, encoding) in table.iter() {
        println!("{}: {}", *ch, encoding);
    }
}
