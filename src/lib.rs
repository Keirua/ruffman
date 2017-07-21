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
}

#[derive(Debug)]
pub enum NodeType {
    Node,
    Leaf
}

#[derive(Debug)]
pub struct HuffmanNode<'a> {
    pub node_type: NodeType,
    pub key: char,
    pub value: i32,
    left: Option<&'a HuffmanNode<'a>>,
    right: Option<&'a HuffmanNode<'a>>,
}

impl<'a> HuffmanNode<'a> {
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
            right: Some(&left),
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
}
