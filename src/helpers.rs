use num_traits::{PrimInt, WrappingAdd, WrappingShl, WrappingShr};

/// Extracts a range of bits from a number (u8, u16, u32, i8, i16, i32)
/// 
/// # Arguments
/// 
/// * `number` - The number to extract bits from
/// * `start_bit` - The starting bit position (0-indexed)
/// * `end_bit` - The ending bit position (0-indexed)
/// 
/// # Returns
/// 
/// * `Ok(u32)` - The extracted bits
/// * `Err(&'static str)` - Error message
/// 
/// # Examples
/// 
/// ```
/// let number = 0b1111_1111_1111_1111_1111_1111_1111_1010u32;
/// let result = extract_bits(number, 0, 3);
/// assert_eq!(result, Ok(0b1010));
/// ```
/// 
pub fn extract_bits<T>(number: T, start_bit: u32, end_bit: u32) -> Result<T, &'static str> 
where
    T: PrimInt + WrappingAdd + WrappingShl + WrappingShr,
{
    let type_bits = std::mem::size_of::<T>() * 8;
    let type_bits = u32::try_from(type_bits).unwrap();

    if start_bit > end_bit {
        return Err("Start position must be less than or equal to end position");
    }
    if end_bit >= type_bits {
        return Err("End position must be less than the type's bit width");
    }

    let num_bits = end_bit - start_bit + 1;
    
    // Create a mask of appropriate length
    let mask = if num_bits == type_bits {
        T::max_value()
    } else {
        (T::one() << num_bits.try_into().unwrap()) - T::one()
    };
    
    // Shift right to remove lower bits, then apply mask
    let result = (number >> start_bit.try_into().unwrap()) & mask;
    
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_bits_u8() {
        assert_eq!(extract_bits(0b1111_1010u8, 0, 3).unwrap(), 0b1010u8);
    }

    #[test]
    fn test_extract_bits_u16() {
        assert_eq!(extract_bits(0b1111_1111_1010_1010u16, 0, 7).unwrap(), 0b10101010u16);
    }

    #[test]
    fn test_extract_bits_u32() {
        assert_eq!(extract_bits(0b1111_1111_1111_1111_1010_1010_1010_1010u32, 0, 15).unwrap(), 0b1010101010101010u32);
    }

    #[test]
    fn test_extract_bits_i8() {
        assert_eq!(extract_bits(0b1010i8, 0, 1).unwrap(), 0b10i8);
    }

    #[test]
    fn test_extract_bits_i16() {
        assert_eq!(extract_bits(0b1111_1010i16, 0, 3).unwrap(), 0b1010i16);
    }

    #[test]
    fn test_extract_bits_i32() {
        assert_eq!(extract_bits(0b1111_1111_1010_1010i32, 0, 7).unwrap(), 0b1010_1010i32);
    }

    #[test]
    fn test_extract_bits_invalid_start() {
        assert_eq!(extract_bits(0u8, 2, 1), Err("Start position must be less than or equal to end position"));
    }

    #[test]
    fn test_extract_bits_invalid_end() {
        assert_eq!(extract_bits(0u8, 0, 8), Err("End position must be less than the type's bit width"));
    }
    
}