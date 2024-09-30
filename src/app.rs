use crate::compression::huffman::Symbol;

use crate::compression::huffman;
use crate::io::{input::Reader, output::Writer};

use std::collections::HashMap;
use std::error::Error;
use std::io::{BufReader, Read, Seek, Write};

fn read_header(
    reader: &mut Reader<impl Read>,
) -> Result<Vec<(char, huffman::Symbol)>, std::io::Error> {
    let entries_cnt = read_byte(reader)?;
    let mut res: Vec<(char, Symbol)> = vec![];
    // dbg!(entries_cnt);
    for _ in 0..entries_cnt {
        let char = read_byte(reader)? as char;
        let sym_len = read_byte(reader)? as usize;
        let mut sym_data: Vec<bool> = vec![false; sym_len];
        // dbg!(sym_len);
        // FIXME: bug
        let n = reader.read_bits(&mut sym_data)?;
        if n != sym_len {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Interrupted,
                "Read bits are different from symbol length",
            ));
        }
        res.push((char, Symbol::from(sym_data)))
    }

    Ok(res)
}

fn read_byte(reader: &mut Reader<impl Read>) -> Result<u8, std::io::Error> {
    let mut bits = [false; 8];
    let n = reader.read_bits(&mut bits)?;
    if n < 8 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Interrupted,
            "Found malformed byte (n bits < 8)",
        ));
    }
    let mut byte = 0u8;
    // b starts from the most significant bit
    for b in bits {
        byte <<= 1;
        if b {
            // write least significant bit
            byte |= 1;
        }
    }
    // dbg!((bits, byte));
    Ok(byte)
}

fn write_header(
    writer: &mut Writer<impl Write>,
    sym_table: &Vec<(char, huffman::Symbol)>,
) -> Result<(), std::io::Error> {
    write_byte(writer, sym_table.len() as u8)?;
    for (ch, sym) in sym_table {
        write_byte(writer, *ch as u8)?;
        write_byte(writer, sym.data.len() as u8)?;
        write_symbol(writer, &sym)?;
        // dbg!(sym_table, sym);
    }
    Ok(())
}

fn write_symbol(
    writer: &mut Writer<impl Write>,
    sym: &huffman::Symbol,
) -> Result<(), std::io::Error> {
    writer.write_bits(sym.data.as_slice())?;
    Ok(())
}

fn write_byte(writer: &mut Writer<impl Write>, byte: u8) -> Result<(), std::io::Error> {
    let bits: Vec<bool> = (0..8).map(|i| (byte << i) & 128 != 0).collect();
    writer.write_bits(bits.as_slice())?;
    Ok(())
}

fn build_sym_hashmap(header: Vec<(char, Symbol)>) -> HashMap<String, char> {
    let mut hash_map = HashMap::new();
    // dbg!(&header);
    for (c, sym) in header {
        hash_map.insert(
            sym.data
                .iter()
                .map(|d| {
                    if *d {
                        return "1".to_string();
                    }
                    "0".to_string()
                })
                .collect::<String>(),
            c,
        );
    }
    hash_map
}

pub fn decompress_block<R: Read>(
    mut input_reader: &mut Reader<R>,
    mut output_stream: impl Write,
) -> Result<u64, Box<dyn Error>> {
    let header = match read_header(&mut input_reader) {
        Ok(header) => header,
        Err(e) if e.kind() == std::io::ErrorKind::Interrupted => return Ok(0),
        Err(e) => return Err(e.into()),
    };

    let mut total_symbols_count = 0u64;
    for _ in 0..8 {
        let byte = read_byte(&mut input_reader)
            .map_err(|e| format!("error reading symbol count: {}", e))?;
        total_symbols_count >>= 8;
        total_symbols_count |= (byte as u64) << (8 * 7);
    }
    // dbg!(total_symbols_count);

    let mut sym = String::new();
    let hash_map = build_sym_hashmap(header);
    // dbg!(&hash_map);
    let mut read_symbols_count = 0;
    while read_symbols_count < total_symbols_count {
        let mut bits = [false];
        input_reader
            .read_bits(&mut bits)
            .map_err(|e| format!("error reading symbol bit: {}", e))?;
        sym.push(if bits[0] { '1' } else { '0' });
        
        if let Some(&ch) = hash_map.get(&sym) {
            // dbg!(&sym, &ch);
            output_stream.write(&[ch as u8])?;
            output_stream.flush()?;
            sym.clear();
            read_symbols_count += 1;
            // dbg!(&read_symbols_count);
        }
    }

    if read_symbols_count < total_symbols_count {
        return Err("not enough symbols read".into());
    }

    if sym.len() > 0 {
        return Err("symbol buffer is not empty".into());
    }

    Ok(read_symbols_count)
}

pub fn compress_block<InnerStream: Write>(
    input_stream: impl Read + Seek,
    mut output_stream: &mut Writer<InnerStream>,
) -> Result<(), std::io::Error> {
    let mut reader = BufReader::new(input_stream);
    let mut freq_map: HashMap<char, u64> = HashMap::new();

    loop {
        let mut bytes = [0; 1];
        let n = reader.read(&mut bytes)?;
        if n == 0 {
            break;
        }
        let char = bytes[0] as char;

        freq_map.entry(char).and_modify(|e| *e += 1).or_insert(1);
    }

    let mut sym_table = huffman::encode(&freq_map);
    sym_table.sort_unstable_by_key(|(c, _)| *c);

    reader.rewind()?;

    write_header(&mut output_stream, &sym_table)?;

    let symbols_count: u64 = freq_map.iter().map(|(_, freq)| freq).sum();
    let mut symbols_count_bytes = [0u8; 8];
    for i in 0..8 {
        symbols_count_bytes[i] = (symbols_count >> i * 8) as u8;
    }
    for byte in symbols_count_bytes {
        write_byte(&mut output_stream, byte)?;
    }
    // dbg!(symbols_count);

    loop {
        let mut bytes = [0; 1];
        let n = reader.read(&mut bytes)?;
        if n == 0 {
            break;
        }
        let char = bytes[0] as char;

        let sym_id = sym_table.binary_search_by_key(&char, |(c, _)| *c).unwrap();
        let (_, sym) = &sym_table[sym_id];
        write_symbol(&mut output_stream, sym)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::io::input::Reader;

    use super::{compress_block, decompress_block};

    #[test]
    fn should_compress_and_decompress_one_block() {
        let block_to_compress = "ciao".as_bytes();
        let mut compressed_stream = [0u8; 1024];
        let mut bit_writer = crate::io::output::Writer::new(&mut compressed_stream[..]);

        compress_block(Cursor::new(block_to_compress), &mut bit_writer)
            .expect("compress should not throw error");

        let mut decompressed_stream = [0u8; 1024];

        let mut reader_to_decompress = Reader::new(&compressed_stream[..]);
        decompress_block(&mut reader_to_decompress, &mut decompressed_stream[..])
            .expect("decompress should not throw error");

        assert_eq!(
            block_to_compress[..],
            decompressed_stream[..block_to_compress.len()]
        );
    }

    #[test]
    fn should_compress_and_decompress_two_blocks() {
        let first_block_to_compress = "ciao".as_bytes();
        let second_block_to_compress = "mondo".as_bytes();
        let mut compressed_stream = [0u8; 1024];
        let mut bit_writer = crate::io::output::Writer::new(&mut compressed_stream[..]);

        compress_block(Cursor::new(first_block_to_compress), &mut bit_writer)
            .expect("compress should not throw error");
        compress_block(Cursor::new(second_block_to_compress), &mut bit_writer)
            .expect("compress should not throw error");

        bit_writer.flush().expect("should flush bit writer");
        let mut reader_to_decompress = Reader::new(&compressed_stream[..]);

        let mut decompressed_stream = [0u8; 1024];

        let mut decompressed_stream_cursor = Cursor::new(&mut decompressed_stream[..]);

        decompress_block(&mut reader_to_decompress, &mut decompressed_stream_cursor)
            .expect("decompress should not throw error");

        decompress_block(&mut reader_to_decompress, &mut decompressed_stream_cursor)
            .expect("decompress should not throw error");

        assert_eq!(first_block_to_compress[..], decompressed_stream[..4]);

        assert_eq!(second_block_to_compress[..], decompressed_stream[4..9]);
    }
}
