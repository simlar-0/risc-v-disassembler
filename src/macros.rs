/// Extracts bits from a number
/// 
/// # Arguments
/// 
/// * `num` - The number to extract bits from
/// * `start` - The starting index of the bits to extract
/// * `end` - The ending index of the bits to extract
/// 
/// # Returns
/// 
/// * `Ok(u<size>)` - The extracted bits
/// * `Err(&'static str)` - Error message
/// 
/// # Examples
/// 
/// ```
/// use risc_v_disassembler::extract_bits;
/// 
/// let number: u32 = 0b1111_1111_1001_1111_1111_1111_1111_1010;
/// let result = extract_bits!(number, 0, 3);
/// assert_eq!(result, Ok(0b1010));
/// 
/// let result = extract_bits!(number, 4, 7);
/// assert_eq!(result, Ok(0b1111));
/// ```
#[macro_export]
macro_rules! extract_bits {
    ($num:expr, $start:expr, $end:expr) => {{
        let bit_size = std::mem::size_of_val(&$num) * 8;
        if $start <0 || $end >= bit_size {
            Err($crate::DisassemblerError::BitExtractionError("Index out of bounds"))
        } else if $start > $end {
            Err($crate::DisassemblerError::BitExtractionError("Start index must be less than or equal to end index"))
        } else {
            let mask = (1 << ($end - $start + 1)) - 1;
            Ok(($num >> $start) & mask)
        }
    }};
}

pub(crate) use extract_bits;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_bits_from_u64() {
        let number: u64 = 0b1101_1111_1001_1111_1111_1111_1111_1010_1110_1111_1001_1111_1111_1111_1111_1010;
        let result = extract_bits!(number, 0, 3);
        assert_eq!(result, Ok(0b1010));

        let result = extract_bits!(number, 4, 7);
        assert_eq!(result, Ok(0b1111));

        let result = extract_bits!(number, 20, 23);
        assert_eq!(result, Ok(0b1001));

        let result = extract_bits!(number, 24, 31);
        assert_eq!(result, Ok(0b1110_1111));

        let result = extract_bits!(number, 56, 63);
        assert_eq!(result, Ok(0b1101_1111));
    }

    #[test]
    fn test_extract_bits_from_i64() {
        let number: i64 = 0b101_1111_1001_1111_1111_1111_1111_1010_1110_1111_1001_1111_1111_1111_1111_1010;
        let result = extract_bits!(number, 0, 3);
        assert_eq!(result, Ok(0b1010));

        let result = extract_bits!(number, 4, 7);
        assert_eq!(result, Ok(0b1111));

        let result = extract_bits!(number, 20, 23);
        assert_eq!(result, Ok(0b1001));

        let result = extract_bits!(number, 24, 31);
        assert_eq!(result, Ok(0b1110_1111));

        let result = extract_bits!(number, 56, 63);
        assert_eq!(result, Ok(0b101_1111));
    }
    
    #[test]
    fn test_extract_bits_u32() {
        let number: u32 = 0b1011_1111_1001_1111_1111_1111_1111_1010;
        let result = extract_bits!(number, 0, 3);
        assert_eq!(result, Ok(0b1010));

        let result = extract_bits!(number, 4, 7);
        assert_eq!(result, Ok(0b1111));

        let result = extract_bits!(number, 20, 23);
        assert_eq!(result, Ok(0b1001));

        let result = extract_bits!(number, 24, 31);
        assert_eq!(result, Ok(0b1011_1111));
    }

    #[test]
    fn test_extract_bits_i32() {
        let number: u32 = 0b011_1111_1001_1111_1111_1111_1111_1010;
        let result = extract_bits!(number, 0, 3);
        assert_eq!(result, Ok(0b1010));

        let result = extract_bits!(number, 4, 7);
        assert_eq!(result, Ok(0b1111));

        let result = extract_bits!(number, 20, 23);
        assert_eq!(result, Ok(0b1001));

        let result = extract_bits!(number, 24, 31);
        assert_eq!(result, Ok(0b011_1111));
    }

    #[test]
    fn test_extract_bits_u16() {
        let number: u16 = 0b1100_1111_1001_1111;
        let result = extract_bits!(number, 0, 3);
        assert_eq!(result, Ok(0b1111));

        let result = extract_bits!(number, 4, 7);
        assert_eq!(result, Ok(0b1001));

        let result = extract_bits!(number, 8, 11);
        assert_eq!(result, Ok(0b1111));

        let result = extract_bits!(number, 12, 15);
        assert_eq!(result, Ok(0b1100));
    }

    #[test]
    fn test_extract_bits_i16() {
        let number: u16 = 0b100_1111_1001_1111;
        let result = extract_bits!(number, 0, 3);
        assert_eq!(result, Ok(0b1111));

        let result = extract_bits!(number, 4, 7);
        assert_eq!(result, Ok(0b1001));

        let result = extract_bits!(number, 8, 11);
        assert_eq!(result, Ok(0b1111));

        let result = extract_bits!(number, 12, 15);
        assert_eq!(result, Ok(0b100));
    }

    #[test]
    fn test_extract_bits_u8() {
        let number: u8 = 0b1011_1010;
        let result = extract_bits!(number, 0, 3);
        assert_eq!(result, Ok(0b1010));

        let result = extract_bits!(number, 4, 7);
        assert_eq!(result, Ok(0b1011));
    }

    #[test]
    fn test_extract_bits_i8() {
        let number: u8 = 0b011_1010;
        let result = extract_bits!(number, 0, 3);
        assert_eq!(result, Ok(0b1010));

        let result = extract_bits!(number, 4, 7);
        assert_eq!(result, Ok(0b011));
    }

}