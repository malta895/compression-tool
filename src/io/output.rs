use crate::compression::huffman;

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

fn write_symbol(sym: huffman::Symbol, writer: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
    let bytes_chunks = sym.data[..].chunks(8);

    for byte_chunk in bytes_chunks {
        let mut byte = 0 as u8;
        for i in 0..8 {
            byte = byte << 1;
            if i < byte_chunk.len() && byte_chunk[i] {
                byte = byte | 1;
            }
        }
        writer.write(&[byte])?;
    }

    Ok(())
}