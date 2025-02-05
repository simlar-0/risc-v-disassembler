/// Extracts a range of bits from a u32 number
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
/// let number = 0b1111_1111_1111_1111_1111_1111_1111_1010;
/// let result = extract_bits_from_u32(number, 0, 3);
/// assert_eq!(result, Ok(0b1010));
/// ```
/// 
pub fn extract_bits_from_u32(number: u32, start_bit: u32, end_bit: u32) -> Result<u32, &'static str> {
    if start_bit > end_bit {
        return Err("Start position must be less than or equal to end position");
    }
    if end_bit >= 32 {
        return Err("End position must be less than 32");
    }

    let num_bits = end_bit - start_bit + 1;
    
    // Create a mask of appropriate length
    let mask = if num_bits == 32 {
        u32::MAX
    } else {
        (1 << num_bits) - 1
    };
    
    // Shift right to remove lower bits, then apply mask
    let result = (number >> start_bit) & mask;
    
    Ok(result)
}