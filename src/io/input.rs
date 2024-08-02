use std::io::{self, BufReader, Read};
pub struct Reader<R: Read> {
    buf_reader: BufReader<R>,
    buffer: u8,
    buffer_len: usize,
}

impl<R: Read> Reader<R> {
    pub fn new(reader: R) -> Self {
        Self {
            buf_reader: BufReader::new(reader),
            buffer: 0,
            buffer_len: 0,
        }
    }

    pub fn read_bits(&mut self, n: usize) -> Result<Vec<bool>, std::io::Error> {
        let mut bits: Vec<bool> = Vec::with_capacity(n);
        let mut found_first_one = false;
        for _ in 0..n{
            if self.buffer_len == 0{
                let mut byte = [0];
                self.buf_reader.read_exact(&mut byte)?;
                self.buffer = byte[0];
                self.buffer_len = 8;
            }
            self.buffer_len -= 1;
            let shifted_bit = self.buffer >> self.buffer_len;
            let bit = shifted_bit & 1;
            if found_first_one || bit == 1{
                found_first_one = true;
                bits.push(bit==1);
            }
        }
        Ok(bits)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_one_bit_from_byte_array() {
        let data: &[u8] = &[0b1];
        let mut reader = Reader::new(data);
        let bits = reader.read_bits(8).unwrap();
        assert_eq!(bits, vec![true]);
    }

    #[test]
    fn test_read_bits_from_byte_array() {
        let data: &[u8] = &[0b10101010];
        let mut reader = Reader::new(data);
        let bits = reader.read_bits(8).unwrap();
        assert_eq!(bits, vec![true, false, true, false, true, false, true, false]);
    }
}