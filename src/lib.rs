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
