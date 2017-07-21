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

#[derive(Debug)]
enum NodeType {
    Node,
    Leaf
}

#[derive(Debug)]
struct HuffmanNode<'a> {
    node_type: NodeType,
    key: char,
    value: i32,
    left:Option<&'a HuffmanNode<'a>>,
    right:Option<&'a HuffmanNode<'a >>,
}

impl<'a> HuffmanNode<'a> {
    fn new_leaf(key: char, value: i32) -> HuffmanNode<'a> {
        let leaf = HuffmanNode {
            node_type: NodeType::Leaf,
            key: key,
            value: value,
            left: None,
            right: None
        };

        return leaf;
    }

    fn new_node (left:&'a HuffmanNode, right: &'a HuffmanNode) -> HuffmanNode<'a> {
        let node = HuffmanNode {
            node_type: NodeType::Node,
            key: ' ',
            value: left.value + right.value,
            left: Some(left),
            right: Some(left)
        };

        return node;
    }
}

fn main() {
    let original = "abbcccc";
    let mut packer = ruffman::BitPacker::new();

    let bits = vec![1,0,0,0,0,0,0,1,1];
    packer.pack_bits(&bits);

    let hash = count_chars(&String::from(original));

    let a = HuffmanNode::new_leaf('a', 1);
    let b = HuffmanNode::new_leaf('b', 2);
    let c = HuffmanNode::new_leaf('c', 4);

    let ab = HuffmanNode::new_node(&a, &b);
    let root = HuffmanNode::new_node(&ab, &c);

    println!("{:#?}", root);
}
