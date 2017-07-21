extern crate ruffman;

use std::collections::HashMap;
use ruffman::{HuffmanNode, HuffmanDictionnary};

fn find_smallest_node<'a> (nodes : &Vec<HuffmanNode<'a>>) -> usize{
    let mut pos:usize = 0;
    for i in 0..nodes.len() {
        if nodes[pos].value > nodes[i].value {
            pos = i;
        }
    }
    pos
}

fn build_tree<'a>(original: &String) -> HuffmanNode<'a> {
    let hash = ruffman::count_chars(&original);

    let mut nodes: Vec<HuffmanNode<'a>> = hash.into_iter()
        .map(|t| HuffmanNode::new_leaf(t.0, t.1))
        .collect();

    // while nodes.len() > 1 {
    //     let index_a = find_smallest_node(&nodes);
    //     let min_node_a = nodes.remove(index_a);
    //     let index_b = find_smallest_node(&nodes);
    //     let min_node_b = nodes.remove(index_b);
    //
    //     nodes.push(HuffmanNode::new_node(&min_node_a, &min_node_b));
    // }

    nodes.pop().unwrap() // todo: may crash if given an empty string as input
}





fn main() {
    let original = String::from("abbcccc");

    let tree = build_tree(&original);
    println!("{:#?}", tree);

    // let mut packer = ruffman::BitPacker::new();
    //
    // let bits = vec![1,0,0,0,0,0,0,1,1];
    // packer.pack_bits(&bits);

    // let hash = count_chars(&String::from(original));
    //
    let a = HuffmanNode::new_leaf('a', 1);
    let b = HuffmanNode::new_leaf('b', 2);
    let c = HuffmanNode::new_leaf('c', 4);

    let ab = HuffmanNode::new_node(&a, &b);
    let root = HuffmanNode::new_node(&ab, &c);

    println!("{:#?}", root);

    let mut table = HuffmanDictionnary::new();
    table.build_table(&root);
    println!("{:#?}", table.table);
}
