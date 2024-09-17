use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, help = "Compress the input")]
    pub compress: bool,

    #[arg(short, long, help = "Decompress the input")]
    pub decompress: bool,

    #[arg(short, long, help = "Input file name")]
    pub input: String,

    #[arg(
        short,
        long,
        help = "Output file name. If missing, the standard output will be used."
    )]
    pub output: Option<String>,
}

impl Args {
    pub fn validate(mut self) -> Result<Self, &'static str> {
        if !self.compress && !self.decompress {
            self.compress = true;
        }

        if self.compress && self.decompress {
            return Err("Pay for simultaneous compression-decompression");
        }

        Ok(self)
    }
}
