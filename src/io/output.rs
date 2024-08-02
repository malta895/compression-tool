use crate::compression::huffman;
    
pub struct Writer {}

impl Writer {
    pub fn new(writer: &mut dyn std::io::Write) -> Self {
        Self {}
    }

    pub fn write_bits(&mut self, bits: &[bool]) -> Result<(), std::io::Error> {
        Ok(())
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