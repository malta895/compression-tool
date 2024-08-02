
mod compression;
mod io;

use crate::io::output::Writer;
use crate::compression::huffman;

use std::{collections::HashMap, fs::File};
use std::io::{BufReader, Read};

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

    let mut writer = Writer::new(&mut std::io::stdout());

    loop {
        let mut bytes = [0; 1];
        let n = reader.read(&mut bytes)?;
        if n == 0 {
            break;
        }
        let char = bytes[0] as char;
        
        let sym_id = sym_table.binary_search_by_key(&char, |(c, _)| *c).unwrap();
        let (_ , sym) = &sym_table[sym_id];
        writer.write_bits(sym.data.as_slice())?;
    }

    Ok(())
}
    

fn main() {
    let mut args = std::env::args().skip(1);
    let file_path = args.next().unwrap();

    compress_file(file_path.as_str()).unwrap()
}
