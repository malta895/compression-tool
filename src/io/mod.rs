pub mod output;
pub mod input;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn write_read_inversion() {
        let mut bytes = [0; 2];
        
        let expected_bits = [true, false, true, false, false, false, false, false, false, false];
        let mut bit_writer = output::Writer::new(bytes.as_mut_slice());
        bit_writer.write_bits(&expected_bits).expect("should write correctly");
        bit_writer.flush().expect("should flush correctly");

        let mut bit_reader = input::Reader::new(bytes.as_slice());
        let mut actual_bits = [false; 10];
        bit_reader.read_bits(&mut actual_bits).expect("should read correctly");

        assert_eq!(expected_bits, actual_bits);
    }
}