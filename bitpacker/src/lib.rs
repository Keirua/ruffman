
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

    pub fn pack_i32(&mut self, v: u32) {
        for i in 0..32 {
            self.pack_bit((v & ((1<<i) as u32)) as u8)
        }
    }

    pub fn pack_i8(&mut self, v: u8) {
        for i in 0..8 {
            self.pack_bit(v & (1<<i) as u8)
        }
    }

    pub fn pack_bits(&mut self, bits: &Vec<u8>) {
        for b in bits {
            self.pack_bit(*b)
        }
    }

    pub fn flush(&self) -> Vec<u8> {
        let mut bytes = self.packed_bytes.clone();
        if self.current_offset > 0 {
            bytes.push(self.current_byte);
        }

        return bytes;
    }

    pub fn debug(&self) {
        let mut bytes = self.packed_bytes.clone();
        if self.current_offset > 0 {
            bytes.push(self.current_byte);
        }

        println!("# Debug");
        for b in &bytes {
            println!("{:08b} | {:02X} | {:}", b, b, b);
        }

        println!("\t Total length: {}", &bytes.len());
    }
}


pub struct BitUnpacker {
    packed_bytes: Vec<u8>,
    current_byte: usize,
    current_offset: i8
}

impl BitUnpacker {
    pub fn new(packed_bytes: Vec<u8>) -> BitUnpacker{
        BitUnpacker {
            packed_bytes:packed_bytes,
            current_byte: 0,
            current_offset: 0,
        }
    }

    // todo: should deal with the fact that this can overflow
    pub fn read_bits(&mut self, n: i32) -> Vec<u8> {
        let mut reads: Vec<u8> = Vec::new();
        for _ in 0..n {
            let curr_value = (self.packed_bytes[self.current_byte] & (1 << self.current_offset)) != 0;
            self.current_offset += 1;
            reads.push(curr_value as u8);
            if self.current_offset > 7 {
                self.current_byte += 1;
                self.current_offset = 0;
                if self.current_byte >= self.packed_bytes.len() {
                    break;
                }
            }
        }

        reads
    }

    // todo: should deal with the fact that this can overflow
    pub fn peek(&self, n: i32) -> Vec<u8> {
        let mut reads: Vec<u8> = Vec::new();
        let mut current_byte = self.current_byte;
        let mut current_offset = self.current_offset;

        for _ in 0..n {
            let curr_value = (self.packed_bytes[current_byte] & (1 << current_offset)) != 0;
            current_offset += 1;
            if current_offset > 7 {
                current_byte += 1;
                current_offset = 0;
                if current_byte >= self.packed_bytes.len() {
                    break;
                }

            }
            reads.push(curr_value as u8);
        }

        reads
    }

    pub fn read_i32(&mut self) -> i32 {
        let bits = self.read_bits(32);
        let mut result:i32 = 0;
        for (i, v) in bits.iter().enumerate() {
            result |= (*v as i32) << i;
        }

        result as i32
    }

    pub fn read_i8(&mut self) -> i8 {
        let bits = self.read_bits(8);
        let mut result = 0;
        for (i, v) in bits.iter().enumerate() {
            result |= (*v as i8) << i;
        }

        result as i8
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
