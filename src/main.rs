mod args;
mod compression;
mod io;
mod app;

use args::Args;
use clap::Parser;

fn main() {
    let args = Args::parse().validate().expect("Argument validation error");
    dbg!(&args);

    let input_file_name = args.input;
    let output_file_name = args.output;
    if args.compress {
        app::compress_file(&input_file_name.as_str(), output_file_name)
            .expect(format!("Error compressing file {}", input_file_name).as_str());
        return
    }
    if args.decompress {
        app::decompress_file(&input_file_name.as_str(), output_file_name)
            .expect(format!("Error decompressing file {}", input_file_name).as_str());
    }
}
