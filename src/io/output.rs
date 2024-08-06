use crate::compression::huffman;

use std::io::Write;

    
pub struct Writer<'a> {
    writer: &'a mut dyn Write,
    buffer: u8,
    buffer_len: u8,
}

impl Writer {
    pub fn new(writer: &mut dyn std::io::Write) -> Self {
        Self {
            writer,
            buffer: 0,
            buffer_len: 0,
        }
    }

    pub fn write_bits(&mut self, bits: &[bool]) -> Result<(), std::io::Error> {
        for &bit in bits {
            self.buffer = (self.buffer << 1) | (bit as u8);
            self.buffer_len += 1;

            // if self.buffer_len == 8 {
            //     -- flushare il buffer
            // }
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

pub fn write_header(
    sym_table: Vec<(char, huffman::Symbol)>,
    writer: &mut dyn std::io::Write,
) -> Result<(), std::io::Error> {
    writer.write(&[sym_table.len() as u8])?;
    for (ch, sym) in sym_table {
        writer.write(&[ch as u8])?;
        writer.write(&[sym.data.len() as u8])?;
        write_symbol(sym, writer)?;
    }
    Ok(())
}