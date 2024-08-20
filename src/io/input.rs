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

    pub fn read_bits(&mut self, bits: &mut[bool]) -> Result<usize, std::io::Error> {
        let mut n = 0;
        while n < bits.len() {
            if self.buffer_len == 0 {
                let n_read = self.buf_reader.read(&mut[self.buffer])?;
                if n_read == 0 {
                    return Ok(n);
                }
                self.buffer_len = 8;                
            }
            // FIXME: bisogna aggiornare correttamente il buffer e buffer_len
            let first_bit = self.buffer >> self.buffer_len;
            bits[n] = first_bit == 1;


            n +=1;
        }
        Ok(n)
    }
}
