use std::collections::HashMap;

#[derive(Debug)]
pub struct BitPacker {
    packed_bytes: Vec<u8>,
    current_byte: u8,
    current_offset: i8
}

impl BitPacker {
    pub fn new() -> BitPacker {
        let packed_bytes = Vec::new();
        BitPacker {
            current_byte: 0,
            current_offset: 0,
            packed_bytes: packed_bytes
        }
    }

    pub fn pack_bit(&mut self, bit: u8) {
        if bit != 0 {
            self.current_byte |= 1 << self.current_offset
        }
        self.current_offset += 1;
        if self.current_offset > 7 {
            self.current_offset = 0;
            self.packed_bytes.push(self.current_byte);
            self.current_byte = 0;
        }
    }

    pub fn pack_bits(&mut self, bits: &Vec<u8>) {
        for b in bits {
            self.pack_bit(*b)
        }
    }

    pub fn debug(&self) {
        let mut bytes = self.packed_bytes.clone();
        bytes.push(self.current_byte);

        println!("# Debug");
        for b in bytes {
            println!("{:08b} | {:02X} | {:}", b, b, b);
        }
    }
}

#[derive(Debug)]
enum NodeType {
    Node,
    Leaf
}

#[derive(Debug)]
pub struct HuffmanNode<'a> {
    node_type: NodeType,
    pub key: char,
    pub value: i32,
    pub left: Option<&'a HuffmanNode<'a>>,
    pub right: Option<&'a HuffmanNode<'a>>,
}

impl<'a> HuffmanNode<'a> {
    // no == operator is implemented on NodeType, hence the weird if
    pub fn is_leaf(&self) -> bool { if let NodeType::Leaf = self.node_type { true } else { false} }

    pub fn new_leaf(key: char, value: i32) -> HuffmanNode<'a> {
        HuffmanNode {
            node_type: NodeType::Leaf,
            key: key,
            value: value,
            left: None,
            right: None,
        }
    }

    pub fn new_node(left: &'a HuffmanNode, right: &'a HuffmanNode) -> HuffmanNode<'a> {
        HuffmanNode {
            node_type: NodeType::Node,
            key: ' ',
            value: left.value + right.value,
            left: Some(&left),
            right: Some(&right),
        }
    }
}

pub struct HuffmanTree<'a> {
    nodes: Vec<HuffmanNode<'a>>,
    min_node_a :HuffmanNode<'a>,
    min_node_b :HuffmanNode<'a>
}

impl<'a> HuffmanTree<'a> {
    fn find_smallest_node (nodes : &Vec<HuffmanNode<'a>>) -> usize{
        let mut pos:usize = 0;
        for i in 0..nodes.len() {
            if nodes[pos].value > nodes[i].value {
                pos = i;
            }
        }

        pos
    }

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

    pub fn new () -> HuffmanTree<'a> {
        HuffmanTree {
            nodes : vec![],
            min_node_a: HuffmanNode::new_leaf('a', 1),
            min_node_b: HuffmanNode::new_leaf('a', 1),
        }
    }

    pub fn build(&mut self, original: &String) -> &HuffmanNode<'a> {
        println!("tree building");

        let hash = HuffmanTree::count_chars(&original);

        self.nodes = hash.into_iter()
            .map(|t| HuffmanNode::new_leaf(t.0, t.1))
            .collect::<Vec<HuffmanNode<'a>>>();
        &self.nodes[0]
        // let tree : Vec<&HuffmanNode<'a>> = Vec::new();
        // for i in nodes.iter() {
        //     tree.push(&i);
        // }

        // while tree.nodes.len() > 1 {
        //      let index_a = HuffmanTree::find_smallest_node(&tree.nodes);
        //      let min_node_a = tree.nodes[index_a];
        //      tree.nodes.remove(index_a);
        //      let index_b = HuffmanTree::find_smallest_node(&tree.nodes);
        //      tree.min_node_b = tree.nodes.remove(index_b);
        //
        //      tree.nodes.push(HuffmanNode::new_node(&min_node_a, &tree.min_node_b));
        //  }

        // nodes_clone.pop().unwrap() // todo: may crash if given an empty string as input
    }

}

pub struct HuffmanDictionnary {
    pub table: HashMap<char, Vec<u8>>
}

impl HuffmanDictionnary {
    pub fn new () -> HuffmanDictionnary {
        HuffmanDictionnary {
            table: HashMap::new()
        }
    }

    pub fn build_table (&mut self, root: &HuffmanNode)  {
        let v:Vec<u8> = Vec::new();
        println!("table building");
        self.navigate(root, v);
    }

    fn navigate (&mut self, node: &HuffmanNode, v:Vec<u8>) {
        if node.is_leaf() {
            //println!("reached {} {}, {:?}", node.key, node.value, v);
            self.table.entry(node.key).or_insert(v);
        }
        else {
            let mut vl = v.clone();
            vl.push(0);
            let mut vr = v.clone();
            vr.push(1);
            self.navigate(&node.left.unwrap(), vl);
            self.navigate(&node.right.unwrap(), vr);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pack_bit() {
        let mut packer = BitPacker::new();
        packer.pack_bit(1);
        packer.pack_bit(0);
        packer.pack_bit(1);
        packer.pack_bit(1);

        assert_eq!(0, packer.packed_bytes.len());
        assert_eq!(13, packer.current_byte);
        assert_eq!(4, packer.current_offset);
    }

    #[test]
    fn test_pack_bits_array() {
        let mut packer = BitPacker::new();
        let bits = vec![1,0,0,0,0,0,0,0,1,1];
        packer.pack_bits(&bits);

        assert_eq!(1, packer.packed_bytes.len());
        assert_eq!(1, packer.packed_bytes[0]);
        assert_eq!(3, packer.current_byte);
        assert_eq!(2, packer.current_offset);
    }

    #[test]
    fn test_count_chars(){
        let s = String::from("abbcccc");
        let counts = HuffmanTree::count_chars(&s);

        assert_eq!(3, counts.keys().len());
        assert_eq!(1, counts[&'a']);
        assert_eq!(2, counts[&'b']);
        assert_eq!(4, counts[&'c']);
    }

    #[test]
    fn test_building_table(){
        let a = HuffmanNode::new_leaf('a', 1);
        let b = HuffmanNode::new_leaf('b', 2);
        let c = HuffmanNode::new_leaf('c', 4);

        let ab = HuffmanNode::new_node(&a, &b);
        let root = HuffmanNode::new_node(&ab, &c);

        let mut table = HuffmanDictionnary::new();
        table.build_table(&root);

        assert_eq!(3, table.table.keys().len());
        assert_eq!(vec![0, 0], table.table[&'a']);
        assert_eq!(vec![0, 1], table.table[&'b']);
        assert_eq!(vec![1], table.table[&'c']);
    }
}
