// Implements http://rosettacode.org/wiki/LZW_compression

use std::collections::hashmap::HashMap;

// Compress using LZW
fn compress(original_str: &str) -> Vec<int> {
   let original = original_str.as_bytes();
   let mut dict_size = 256;
   let mut dictionary = HashMap::new();

   for i in range(0i, dict_size) {
      dictionary.insert(vec!(i as u8), i);
   }

   let mut result = vec!();
   let mut w = vec!();
   for &c in original.iter() {
      let mut wc = w.clone();
      wc.push(c);

      match dictionary.find(&wc) {
         Some(_) => w = wc,
         None => {
            result.push(dictionary[w]);
            dictionary.insert(wc, dict_size);
            dict_size += 1;
            w = vec!(c);
         }
      }
   }

   if w.len() > 0 {
      result.push(dictionary[w]);
   }

   result
}

// Decompress using LZW
fn decompress(compressed: &Vec<int>) -> String {
   let mut dict_size = 256;
   let mut dictionary = HashMap::new();

   for i in range(0i, dict_size) {
      dictionary.insert(i, vec!(i as u8));
   }

   let mut w = vec!(compressed[0].clone() as u8);
   let compressed = compressed.slice_from(1);
   let mut result = w.clone();
   for &k in compressed.iter() {
      let entry = match dictionary.find(&k) {
          Some(v) => v.clone(),
          None if k == dict_size => { let mut new = w.clone(); new.push(w[0].clone()); new }
          None => fail!("Invalid compressed string")
      };

      result.extend(entry.iter().map(|&x| x.clone()));
      w.push(entry[0].clone());
      dictionary.insert(dict_size, w);
      dict_size += 1;
      w = entry;
   }

   String::from_utf8(result).unwrap()
}

#[cfg(not(test))]
fn main() {
    // Show original
    let original = "TOBEORNOTTOBEORTOBEORNOT";
    println!("Original: {}", original);

    // Show compressed
    let compressed = compress(original);
    println!("Compressed: {}", compressed);

    // Show decompressed
    let decompressed = decompress(&compressed);
    println!("Decompressed: {}", decompressed);
}

#[test]
fn test_coherence() {
    for s in range(50000i, 50100).map(|n| n.to_string()) {
        assert_eq!(decompress(&compress(s.as_slice())), s);
    }
}

#[test]
fn test_example() {
    let original = "TOBEORNOTTOBEORTOBEORNOT";
    assert_eq!(compress(original).as_slice(), [84i, 79, 66, 69, 79, 82, 78, 79, 84,
                                                256, 258, 260, 265, 259, 261, 263].as_slice());
}
