use clap::Parser;

const DEFAULT_BLOCK_BYTES: usize = 4 * 1024; // 4KB

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, help = "Compress the input")]
    pub compress: bool,

    #[arg(short, long, help = "Decompress the input")]
    pub decompress: bool,

    #[arg(short, long, help = "Input file name")]
    pub input: Option<String>,

    #[arg(
        short,
        long,
        help = "Output file name. If missing, the standard output will be used."
    )]
    pub output: Option<String>,

    #[arg(
        short,
        long,
        default_value_t = DEFAULT_BLOCK_BYTES,
        help = "Block size in bytes."
    )]
    pub block_size: usize,
}

impl Args {
    pub fn validate(mut self) -> Result<Self, &'static str> {
        if !self.compress && !self.decompress {
            self.compress = true;
        }

        if self.compress && self.decompress {
            return Err("Pay for simultaneous compression-decompression");
        }

        if self.block_size <= 0 {
            return Err("Block size must be greater than 0");
        }

        Ok(self)
    }
}
