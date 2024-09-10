use std::io::{BufReader, Read};
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

    fn read_bit(&mut self) -> Result<bool, std::io::Error> {
        if self.buffer_len == 0 {
            let mut test_buf = [self.buffer];
            let n_read = self.buf_reader.read(&mut test_buf)?;
            dbg!((test_buf, n_read));
            self.buffer = test_buf[0];
            if n_read == 0 {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Interrupted,
                    "Could not read buffer",
                ));
            }
            dbg!(self.buffer);
            self.buffer_len += 8;
        }

        let bit = self.buffer & 1 == 1;

        self.buffer_len -= 1;
        self.buffer >>= 1;

        Ok(bit)
    }

    pub fn read_bits(&mut self, bits: &mut [bool]) -> Result<usize, std::io::Error> {
        let mut n = 0;
        for idx in (0..bits.len()) {
            let bit = self.read_bit()?;
            dbg!(bit);
            bits[idx] = bit;
            n+=1;
        }
        Ok(n)
    }
}
