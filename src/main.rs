extern crate ruffman;

use std::collections::HashMap;

/// Returns a HashTable whose keys are the characters in the original string, and whose values
/// is the number of times it appears in the string
/// ex: "abbcccc" -> {'c': 4, 'b': 2, 'a': 1}
fn count_chars(original: &String) -> HashMap<char, i32> {
    let mut map: HashMap<char, i32> = HashMap::new();
    for key in original.chars() {
        let count = map.entry(key).or_insert(0);
        *count += 1;
    }

    map
}

fn main() {
    let original = "abbcccc";
    let mut packer = ruffman::BitPacker::new();

    let bits = vec![1,0,0,0,0,0,0,1,1];
    packer.pack_bits(&bits);

    let hash = count_chars(&String::from(original));
    println!("original: {}", &original);
    println!("{:?}", packer);
    println!("{:?}", hash);
}
