use crate::compression::huffman::Symbol;

use crate::compression::huffman;
use crate::io::{input::Reader, output::Writer};

use std::io::{BufReader, BufWriter, Read, Write};
use std::{collections::HashMap, fs::File};

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

pub fn decompress_file(
    input_file_path: &str,
    output_file_path: Option<String>,
) -> Result<(), std::io::Error> {
    let file = File::open(input_file_path)?;
    let mut bit_reader = Reader::new(file);
    let header = read_header(&mut bit_reader)?;

    let mut total_symbols_count = 0u64;
    for _ in 0..8 {
        let byte = read_byte(&mut bit_reader)?;
        total_symbols_count >>= 8;
        total_symbols_count |= (byte as u64) << (8 * 7);
    }
    // dbg!(total_symbols_count);

    let mut output_stream: Box<dyn Write> = if let Some(output_file_path) = output_file_path {
        let file = File::create(output_file_path)?;
        Box::new(BufWriter::new(file))
    } else {
        Box::new(std::io::stdout())
    };
    let mut sym = String::new();
    let hash_map = build_sym_hashmap(header);
    // dbg!(&hash_map);
    let mut read_symbols_count = 0;
    while read_symbols_count < total_symbols_count {
        let mut bits = [false];
        if let 0 = bit_reader.read_bits(&mut bits).unwrap_or(0) {
            if sym.is_empty() {
                output_stream.flush()?;
                return Ok(());
            }
            return Err(std::io::Error::new(
                std::io::ErrorKind::Interrupted,
                "Could not read empty symbol",
            ));
        }
        sym.push(if bits[0] { '1' } else { '0' });
        // dbg!(&sym);

        if let Some(&char) = hash_map.get(&sym) {
            output_stream.write(&[char as u8])?;
            sym.clear();
            read_symbols_count += 1;
        }
    }

    Ok(())
}

pub fn compress_file(
    input_file_path: &str,
    output_file_path: Option<String>,
) -> Result<(), std::io::Error> {
    let file = File::open(input_file_path)?;
    let mut reader = BufReader::new(file);
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

    let file = File::open(input_file_path)?;
    let mut reader = BufReader::new(file);

    let file_writer: Box<dyn Write> = if let Some(output_file_path) = output_file_path {
        let file = File::create(output_file_path)?;
        Box::new(file)
    } else {
        Box::new(std::io::stdout())
    };

    let mut bit_writer = Writer::new(file_writer);
    write_header(&mut bit_writer, &sym_table)?;

    let symbols_count: u64 = freq_map.iter().map(|(_, freq)| freq).sum();
    let mut symbols_count_bytes = [0u8; 8];
    for i in 0..8 {
        symbols_count_bytes[i] = (symbols_count >> i * 8) as u8;
    }
    for byte in symbols_count_bytes {
        write_byte(&mut bit_writer, byte)?;
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
        write_symbol(&mut bit_writer, sym)?;
    }

    bit_writer.flush()
}
