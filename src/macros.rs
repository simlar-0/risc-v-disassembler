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
/// * `Err(DisassemblerError)` - Error message
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

/// Sign-extends a number from chosen sign bit to 32 bits
/// 
/// # Arguments
/// 
/// * `num` - The number to sign-extend
/// * `curr_size` - The number of bits to consider for sign extension
/// 
/// # Returns
/// 
/// * `Ok(i32)` - The sign-extended number
/// * `Err(DisassemblerError)` - Error message
/// 
/// # Examples
/// 
/// ```
/// use risc_v_disassembler::sign_extend32;
/// 
/// let bits: u32 = 0b0010;
/// let result = sign_extend32!(bits, 3).unwrap();
/// assert_eq!(result, 2);
/// 
/// let bits: u32 = 0b1010;
/// let result = sign_extend32!(bits, 4).unwrap();
/// assert_eq!(result, -6);
/// ```
/// 
#[macro_export]
macro_rules! sign_extend32 {
    ($num:expr, $curr_size:expr) => {{
        if $curr_size > 32 {
            Err($crate::DisassemblerError::BitExtensionError("Size exceeds 32 bits"))
        } else {
            let sign_bit: u32 = 1 << ($curr_size - 1);
            let mask: u32 = (1 << $curr_size - 1) - 1; 
            let sign_extended: u32 = if $num & sign_bit != 0 {
                ($num as u32) | !mask
            } else {
                $num
            };
            Ok(sign_extended as i32)
        }
    }}; 
}

pub(crate) use {
    extract_bits,
    sign_extend32,
};


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

    #[test]
    fn test_sign_extend32(){
        let bits: u32 = 0b0010;
        let result = sign_extend32!(bits, 3).unwrap();
        assert_eq!(result, 2);

        let bits: u32 = 0b1010;
        let result = sign_extend32!(bits, 4).unwrap();
        assert_eq!(result, -6);

        let bits: u32 = 0b1010;
        let result = sign_extend32!(bits, 5).unwrap();
        assert_eq!(result, 10);
    }

}