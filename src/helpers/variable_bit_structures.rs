/// Represents an integer with a variable number of bits, 
/// which can be interpreted as both signed and unsigned.
/// 
#[derive(Copy, Clone)]
pub struct VarBitInt {
    bits: u128,
    size: u8,
}

impl VarBitInt {
    pub fn new (bits: u128, size: u8) -> Self {
        if size > 128 {
            panic!("Size of VarBit cannot be greater than 128 bits");
        }
        VarBitInt {
            bits,
            size,
        }
    }
}

impl TryInto<i128> for VarBitInt {
    type Error = &'static str;

    fn try_into(self) -> Result<i128, Self::Error> {
        if self.size > 128 {
            return Err("Cannot convert a VarBit with size greater than 128 bits to i128");
        }
        if self.bits & (1 << self.size - 1) != 0 {
            let mask = u128::MAX << self.size;
            Ok((self.bits | mask) as i128)
        } else {
            Ok(self.bits as i128)
        } 
    }
}

impl TryInto<i64> for VarBitInt {
    type Error = &'static str;
    
    fn try_into(self) -> Result<i64, Self::Error> {
        if self.size > 64 {
            return Err("Cannot convert a VarBit with size greater than 64 bits to i64");
        }
        
        if self.bits & (1 << (self.size - 1)) != 0 {
            let mask = u128::MAX << self.size;
            Ok((self.bits | mask) as i64)
        } else {
            Ok(self.bits as i64)
        }
    }
}


impl TryInto<i32> for VarBitInt {
    type Error = &'static str;
    
    fn try_into(self) -> Result<i32, Self::Error> {
        if self.size > 32 {
            return Err("Cannot convert a VarBit with size greater than 32 bits to i32");
        }
        
        if self.bits & (1 << (self.size - 1)) != 0 {
            let mask = u128::MAX << self.size;
            Ok((self.bits | mask) as i32)
        } else {
            Ok(self.bits as i32)
        }
    }
}

impl TryInto<i16> for VarBitInt {
    type Error = &'static str;
    
    fn try_into(self) -> Result<i16, Self::Error> {
        if self.size > 16 {
            return Err("Cannot convert a VarBit with size greater than 16 bits to i16");
        }
        
        if self.bits & (1 << (self.size - 1)) != 0 {
            let mask = u128::MAX << self.size;
            Ok((self.bits | mask) as i16)
        } else {
            Ok(self.bits as i16)
        }
    }
}

impl TryInto<i8> for VarBitInt {
    type Error = &'static str;
    
    fn try_into(self) -> Result<i8, Self::Error> {
        if self.size > 8 {
            return Err("Cannot convert a VarBit with size greater than 8 bits to i8");
        }
        
        if self.bits & (1 << (self.size - 1)) != 0 {
            let mask = u128::MAX << self.size;
            Ok((self.bits | mask) as i8)
        } else {
            Ok(self.bits as i8)
        }
    }
}

impl TryInto<u128> for VarBitInt {
    type Error = &'static str;
    
    fn try_into(self) -> Result<u128, Self::Error> {
        if self.size > 128 {
            return Err("Cannot convert a VarBit with size greater than 128 bits to u128");
        }
        
        Ok(self.bits)
    }
}

impl TryInto<u64> for VarBitInt {
    type Error = &'static str;
    
    fn try_into(self) -> Result<u64, Self::Error> {
        if self.size > 64 {
            return Err("Cannot convert a VarBit with size greater than 64 bits to u64");
        }
        
        Ok(self.bits as u64)
    }
}

impl TryInto<u32> for VarBitInt {
    type Error = &'static str;
    
    fn try_into(self) -> Result<u32, Self::Error> {
        if self.size > 32 {
            return Err("Cannot convert a VarBit with size greater than 32 bits to u32");
        }
        
        Ok(self.bits as u32)
    }
}

impl TryInto<u16> for VarBitInt {
    type Error = &'static str;
    
    fn try_into(self) -> Result<u16, Self::Error> {
        if self.size > 16 {
            return Err("Cannot convert a VarBit with size greater than 16 bits to u16");
        }
        
        Ok(self.bits as u16)
    }
}

impl TryInto<u8> for VarBitInt {
    type Error = &'static str;
    
    fn try_into(self) -> Result<u8, Self::Error> {
        if self.size > 8 {
            return Err("Cannot convert a VarBit with size greater than 8 bits to u8");
        }
        
        Ok(self.bits as u8)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_varbit_into_8() {
        let varbit_s = VarBitInt::new(0b00101,3);

        let u: u8 = varbit_s.try_into().unwrap();
        assert_eq!(u, 5);

        let i: i8 = varbit_s.try_into().unwrap();
        assert_eq!(i, -3);

        let varbit_u = VarBitInt::new(0b00101,4);

        let u: u8 = varbit_u.try_into().unwrap();
        assert_eq!(u, 5);

        let i: i8 = varbit_u.try_into().unwrap();
        assert_eq!(i, 5);
    }

    #[test]
    fn test_varbit_into_16() {
        let varbit_s = VarBitInt::new(0b0000_0011_1000_0101,10);

        let u: u16 = varbit_s.try_into().unwrap();
        assert_eq!(u, 901);

        let i: i16 = varbit_s.try_into().unwrap();
        assert_eq!(i, -123);

        let varbit_u = VarBitInt::new(0b0000_0011_1000_0101,13);

        let u: u16 = varbit_u.try_into().unwrap();
        assert_eq!(u, 901);

        let i: i16 = varbit_u.try_into().unwrap();
        assert_eq!(i, 901);
    }

    #[test]
    fn test_varbit_into_32() {
        let varbit_s = VarBitInt::new(0x5bed5ed, 27);

        let u: u32 = varbit_s.try_into().unwrap();
        assert_eq!(u, 96_392_685);

        let i: i32 = varbit_s.try_into().unwrap();
        assert_eq!(i, -37_825_043);

        let varbit_u = VarBitInt::new(0x5bed5ed, 28);

        let u: u32 = varbit_u.try_into().unwrap();
        assert_eq!(u, 96_392_685);

        let i: i32 = varbit_u.try_into().unwrap();
        assert_eq!(i, 96_392_685);
    }

    #[test]
    fn test_varbit_into_64() {
        let varbit_s = VarBitInt::new(0x5bed5ed2f34cdf2, 59);

        let u: u64 = varbit_s.try_into().unwrap();
        assert_eq!(u, 414_003_430_440_619_506);

        let i: i64 = varbit_s.try_into().unwrap();
        assert_eq!(i, -162_457_321_862_803_982);

        let varbit_u = VarBitInt::new(0x5bed5ed2f34cdf2, 60);

        let u: u64 = varbit_u.try_into().unwrap();
        assert_eq!(u, 414_003_430_440_619_506);

        let i: i64 = varbit_u.try_into().unwrap();
        assert_eq!(i, 414_003_430_440_619_506);
    }

    #[test]
    fn test_varbit_into_128() {
        let varbit_s = VarBitInt::new(0x5bed5ed2fd34cdff25fe1dc25ed3325, 123);

        let u: u128 = varbit_s.try_into().unwrap();
        assert_eq!(u, 0x5bed5ed2fd34cdff25fe1dc25ed3325);

        let i: i128 = varbit_s.try_into().unwrap();
        assert_eq!(i, -0x2412A12D02CB3200DA01E23DA12CCDB);

        let varbit_u = VarBitInt::new(0x5bed5ed2fd34cdff25fe1dc25ed3325, 124);

        let u: u128 = varbit_u.try_into().unwrap();
        assert_eq!(u, 0x5bed5ed2fd34cdff25fe1dc25ed3325);

        let i: i128 = varbit_u.try_into().unwrap();
        assert_eq!(i, 0x5bed5ed2fd34cdff25fe1dc25ed3325);
    }
}
