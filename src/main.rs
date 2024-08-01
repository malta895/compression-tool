use std::{collections::HashMap, io::Read};

use output::write_header;

mod huffman {
    use std::{
        cmp::Ordering,
        collections::{BinaryHeap, HashMap},
    };

    #[derive(PartialEq, PartialOrd, Eq)]
    enum Tree {
        Leaf {
            data: char,
            freq: u64,
        },
        Node {
            freq: u64,
            left: Box<Tree>,
            right: Box<Tree>,
        },
    }

    impl Tree {
        fn freq(&self) -> u64 {
            match self {
                Tree::Leaf { freq, .. } => *freq,
                Tree::Node { freq, .. } => *freq,
            }
        }
    }

    impl Ord for Tree {
        fn cmp(&self, other: &Self) -> Ordering {
            self.freq().cmp(&other.freq())
        }
    }

    #[derive(Clone, Debug)]
    pub struct Symbol {
        pub data: Vec<bool>,
    }

    impl Symbol {
        fn new() -> Symbol {
            Symbol { data: vec![] }
        }

        fn append(&self, unit: bool) -> Symbol {
            Symbol {
                data: vec![self.data.clone(), vec![unit]].concat(),
            }
        }
    }

    pub fn encode(freq_table: &HashMap<char, u64>) -> Vec<(char, Symbol)> {
        let forest = freq_table
            .iter()
            .map(|(ch, freq)| Tree::Leaf {
                data: *ch,
                freq: *freq,
            })
            .collect::<Vec<Tree>>();
        let mut heap: BinaryHeap<Tree> = BinaryHeap::from(forest);

        let mut tree: Option<Tree> = None;

        loop {
            let t1 = match heap.pop() {
                None => break,
                Some(t1) => t1,
            };

            let t2 = match heap.pop() {
                None => {
                    tree = Some(t1);
                    break;
                }
                Some(t2) => t2,
            };

            let t = Tree::Node {
                freq: t1.freq() + t2.freq(),
                left: Box::new(t1),
                right: Box::new(t2),
            };

            heap.push(t);
        }

        fn build_symbol_table(tree: Tree, path: Option<Symbol>) -> Vec<(char, Symbol)> {
            let path = path.unwrap_or(Symbol::new());
            match tree {
                Tree::Leaf { data, .. } => vec![(data, path)],
                Tree::Node { left, right, .. } => vec![
                    build_symbol_table(*left, Some(path.append(true))),
                    build_symbol_table(*right, Some(path.append(false))),
                ]
                .concat(),
            }
        }

        match tree {
            None => vec![],
            Some(t) => build_symbol_table(t, None),
        }
    }
}

mod output {
    use crate::huffman::Symbol;

    fn write_symbol(sym: Symbol, writer: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
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

    pub fn write_header(sym_table: Vec<(char, Symbol)>, writer: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
        writer.write(&[sym_table.len() as u8])?;
        for (ch, sym) in sym_table {
            writer.write(&[ch as u8])?;
            writer.write(&[sym.data.len() as u8])?;
            write_symbol(sym, writer)?;
        }
        Ok(())
    }
}

fn main() {
    let mut args = std::env::args().skip(1);

    let mut map: HashMap<char, u64> = std::collections::HashMap::new();

    if let Some(filename) = args.next() {
        let file_buf_reader_res =
            std::fs::File::open(filename).map(|file| std::io::BufReader::new(file));

        let mut buf_reader = match file_buf_reader_res {
            Ok(reader) => reader,
            Err(e) => {
                eprintln!("Error while reading file: {}", e);
                print_usage();
                std::process::exit(1);
            }
        };

        let mut buf: [u8; 1024] = [0; 1024];
        loop {
            let read = match buf_reader.read(&mut buf) {
                Ok(n) => n,
                Err(e) => {
                    eprintln!("Error while reading file: {}", e);
                    break;
                }
            };
            let buf = &buf[..read];
            for byte in buf {
                let count = map.entry(*byte as char).or_insert(0);
                *count += 1;
            }
            if read < 1024 {
                break;
            }
        }
        println!("{:#?}", map);
    } else {
        print_usage();
        std::process::exit(1);
    }

    let sym_table = huffman::encode(&map);
    println!("{:#?}", sym_table);

    write_header(sym_table, &mut std::io::stdout()).unwrap()

}

fn print_usage() {
    eprintln!("Usage: cargo run <filename>");
}
