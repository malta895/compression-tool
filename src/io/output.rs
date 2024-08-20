use std::io::Write;

    
pub struct Writer<W: Write> {
    writer: W,
    buffer: u8,
    buffer_len: u8,
}

impl<W: Write> Writer<W>  {
    pub fn new(writer: W) -> Self {
        Self {
            writer,
            buffer: 0,
            buffer_len: 0,
        }
    }

    pub fn flush(&mut self) -> Result<(), std::io::Error> {
        if self.buffer_len > 0 {
            let padding = 8 - self.buffer_len;
            self.buffer <<= padding;
            self.writer.write(&[self.buffer])?;
            self.buffer = 0;
            self.buffer_len = 0;
        }

        Ok(())
    }

    pub fn write_bits(&mut self, bits: &[bool]) -> Result<(), std::io::Error> {
        for &bit in bits {
            self.buffer = (self.buffer << 1) | (bit as u8);
            self.buffer_len += 1;

            if self.buffer_len == 8 {
                self.writer.write(&[self.buffer])?;
                self.buffer = 0;
                self.buffer_len = 0;
            }
        }


        Ok(())  // il prezzo è giusto
    }
}