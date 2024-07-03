use std::{collections::HashMap, io::Read};

fn main() {
    let mut args = std::env::args()
        .skip(1);

    if let Some(filename) = args.next() {
        let file_buf_reader_res = std::fs::File::open(filename)
        .map(|file| {
            std::io::BufReader::new(file)
        });

       let mut buf_reader = match file_buf_reader_res {
            Ok(reader) => reader,
            Err(e) => {
                eprintln!( "Error while reading file: {}", e);
                print_usage();
                std::process::exit(1);
            }
        };

        let mut buf:[u8;1024] = [0;1024];
        let mut map:HashMap<char, u64>= std::collections::HashMap::new();
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
                let count = map
                .entry(*byte as char)
                .or_insert(0);
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
}



fn print_usage() {
    eprintln!("Usage: cargo run <filename>");
}