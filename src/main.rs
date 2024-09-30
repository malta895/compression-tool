mod app;
mod args;
mod compression;
mod io;

use std::{
    fs::File,
    io::{Cursor, Read, Write},
};

use args::Args;
use clap::Parser;
use io::input::Reader;

fn main() {
    let args = Args::parse().validate().expect("Argument validation error");
    dbg!(&args);

    let mut input_stream: Box<dyn Read> = if let Some(input_file) = args.input {
        Box::new(File::open(input_file).expect("Error opening input file"))
    } else {
        Box::new(std::io::stdin())
    };

    let mut output_stream: Box<dyn Write> = if let Some(output_file) = args.output {
        Box::new(File::create(output_file).expect("Error creating output file"))
    } else {
        Box::new(std::io::stdout())
    };

    if args.compress {
        let mut input_block = vec![0u8; args.block_size];
        let mut bit_writer = crate::io::output::Writer::new(&mut output_stream);

        loop {
            let n = input_stream
                .read(&mut input_block)
                .expect("Error reading from stdin");

            if n == 0 {
                bit_writer.flush().expect("Error flushing bit writer");
                return;
            }

            app::compress_block(Cursor::new(&input_block[..n]), &mut bit_writer)
                .expect("Error compressing block");
        }

        // if let Some(input_file) = args.input {
        //     let input_stream = File::open(input_file.as_str()).expect("Error opening input file");

        //     app::compress_block(input_stream, &mut output_stream)
        //         .expect(format!("Error compressing file {}", input_file).as_str());
        //     return;
        // }
    }

    if args.decompress {
        let mut input_reader = Reader::new(input_stream);
        loop {
            match app::decompress_block(&mut input_reader, &mut output_stream) {
                Ok(()) => (),
                Err(err) if err.kind() == std::io::ErrorKind::Interrupted => return,
                Err(err) => panic!("Error decompressing block: {}", err),
            }
        }
    }
}
