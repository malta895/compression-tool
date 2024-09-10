mod compression;
mod io;

use compression::huffman::Symbol;

use crate::compression::huffman;
use crate::io::{input::Reader, output::Writer};

use std::io::{BufReader, Bytes, Read, Write};
use std::process::{exit, ExitCode};
use std::{collections::HashMap, fs::File};

fn read_header(
    reader: &mut Reader<impl Read>,
) -> Result<Vec<(char, huffman::Symbol)>, std::io::Error> {
    let entries_cnt = read_byte(reader)?;
    let mut res: Vec<(char, Symbol)> = vec![];
    dbg!(entries_cnt);
    for _ in 0..entries_cnt {
        let char = read_byte(reader)? as char;
        let sym_len = read_byte(reader)? as usize;
        let mut sym_data: Vec<bool> = vec![false; sym_len];
        dbg!(sym_len);
        // FIXME: bug
        let n = reader.read_bits(&mut sym_data)?;
        if n != sym_len {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Interrupted,
                "Read bits are different from symbol length"
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
    for b in bits {
        byte >>= 1;
        
        if b {
            // write most significant bit
            byte |= 128;
        }
    }
    dbg!((bits, byte));
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
        dbg!(sym_table, sym);
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
    dbg!(&header);
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

fn decompress_file(file_path: &str) -> Result<(), std::io::Error> {
    let file = File::open(file_path)?;
    let mut bit_reader = Reader::new(file);
    let header = read_header(&mut bit_reader)?;

    let mut stdout = std::io::stdout();
    let mut sym = String::new();
    let hash_map = build_sym_hashmap(header);
    dbg!(&hash_map);
    loop {
        dbg!(&sym);
        let bit = false;
        if let 0 = bit_reader.read_bits(&mut [bit]).unwrap_or(0) {
            if sym.is_empty() {
                stdout.flush()?;
                return Ok(())
            }
            return Err(std::io::Error::new(
                std::io::ErrorKind::Interrupted,
                "Could not read empty symbol",
            ));
            
        }
        sym.push(if bit { '1' } else { '0' });

        if let Some(&char) = hash_map.get(&sym) {
            stdout.write(&[char as u8])?;
            sym.clear();
        }
    }
}

fn compress_file(file_path: &str) -> Result<(), std::io::Error> {
    let file = File::open(file_path)?;
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

    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);

    let mut writer = Writer::new(std::io::stdout());

    write_header(&mut writer, &sym_table)?;
    loop {
        let mut bytes = [0; 1];
        let n = reader.read(&mut bytes)?;
        if n == 0 {
            break;
        }
        let char = bytes[0] as char;

        let sym_id = sym_table.binary_search_by_key(&char, |(c, _)| *c).unwrap();
        let (_, sym) = &sym_table[sym_id];
        write_symbol(&mut writer, sym)?;
    }

    writer.flush()
}

fn main() {
    let mut args = std::env::args().skip(1);
    let file_path = args.last().unwrap();
    let mut args = std::env::args().skip(1);
    if args.any(|a|{a == "-c"}) {
        compress_file(file_path.as_str()).unwrap();
        return;
    }
    let mut args = std::env::args().skip(1);
    if args.any(|a|{a == "-d"}) {
        if let Err(err) = decompress_file(file_path.as_str()){
            eprintln!("Error decompressing: {}", err);
            exit(1);
        }
        return;
    }

    eprintln!("Wrong args");
}
