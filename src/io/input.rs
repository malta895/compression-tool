pub struct Reader {}

impl Reader {
    pub fn new(reader: &mut dyn std::io::Read) -> Self {
        Self {}
    }

    pub fn read_bits(&mut self, n: usize) -> Result<Vec<bool>, std::io::Error> {
        Ok(vec![])
    }
}