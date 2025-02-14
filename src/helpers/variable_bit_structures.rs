use crate::DisassemblerError;

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

impl TryFrom<VarBitInt> for i128 {
    type Error = DisassemblerError;

    fn try_from(value: VarBitInt) -> Result<Self, Self::Error> {
        if value.size > 128 {
            return Err(DisassemblerError::VarBitSizeExceeded(value.size, "i128"));
        }
        if value.bits & (1 << value.size - 1) != 0 {
            let mask = u128::MAX << value.size;
            Ok((value.bits | mask) as i128)
        } else {
            Ok(value.bits as i128)
        } 
    }
}

impl TryFrom<VarBitInt> for i64 {
    type Error = DisassemblerError;
    
    fn try_from(value: VarBitInt) -> Result<Self, Self::Error> {
        if value.size > 64 {
            return Err(DisassemblerError::VarBitSizeExceeded(value.size, "i64"));
        }
        
        if value.bits & (1 << (value.size - 1)) != 0 {
            let mask = u128::MAX << value.size;
            Ok((value.bits | mask) as i64)
        } else {
            Ok(value.bits as i64)
        }
    }
}

impl TryFrom<VarBitInt> for i32 {
    type Error = DisassemblerError;
    
    fn try_from(value: VarBitInt) -> Result<Self, Self::Error> {
        if value.size > 32 {
            return Err(DisassemblerError::VarBitSizeExceeded(value.size, "i32"));
        }
        
        if value.bits & (1 << (value.size - 1)) != 0 {
            let mask = u128::MAX << value.size;
            Ok((value.bits | mask) as i32)
        } else {
            Ok(value.bits as i32)
        }
    }
}

impl TryFrom<VarBitInt> for i16 {
    type Error = DisassemblerError;
    
    fn try_from(value: VarBitInt) -> Result<Self, Self::Error> {
        if value.size > 16 {
            return Err(DisassemblerError::VarBitSizeExceeded(value.size, "i16"));
        }
        
        if value.bits & (1 << (value.size - 1)) != 0 {
            let mask = u128::MAX << value.size;
            Ok((value.bits | mask) as i16)
        } else {
            Ok(value.bits as i16)
        }
    }
}

impl TryFrom<VarBitInt> for i8 {
    type Error = DisassemblerError;
    
    fn try_from(value: VarBitInt) -> Result<Self, Self::Error> {
        if value.size > 8 {
            return Err(DisassemblerError::VarBitSizeExceeded(value.size, "i8"));
        }
        
        if value.bits & (1 << (value.size - 1)) != 0 {
            let mask = u128::MAX << value.size;
            Ok((value.bits | mask) as i8)
        } else {
            Ok(value.bits as i8)
        }
    }
}

impl TryFrom<VarBitInt> for u128 {
    type Error = DisassemblerError;
    
    fn try_from(value: VarBitInt) -> Result<Self, Self::Error> {
        if value.size > 128 {
            return Err(DisassemblerError::VarBitSizeExceeded(value.size, "u128"));
        }
        
        Ok(value.bits)
    }
}

impl TryFrom<VarBitInt> for u64 {
    type Error = DisassemblerError;
    
    fn try_from(value: VarBitInt) -> Result<Self, Self::Error> {
        if value.size > 64 {
            return Err(DisassemblerError::VarBitSizeExceeded(value.size, "u64"));
        }
        
        Ok(value.bits as u64)
    }
}

impl TryFrom<VarBitInt> for u32 {
    type Error = DisassemblerError;
    
    fn try_from(value: VarBitInt) -> Result<Self, Self::Error> {
        if value.size > 32 {
            return Err(DisassemblerError::VarBitSizeExceeded(value.size, "u32"));
        }
        
        Ok(value.bits as u32)
    }
}

impl TryFrom<VarBitInt> for u16 {
    type Error = DisassemblerError;
    
    fn try_from(value: VarBitInt) -> Result<Self, Self::Error> {
        if value.size > 16 {
            return Err(DisassemblerError::VarBitSizeExceeded(value.size, "u16"));
        }
        
        Ok(value.bits as u16)
    }
}

impl TryFrom<VarBitInt> for u8 {
    type Error = DisassemblerError;
    
    fn try_from(value: VarBitInt) -> Result<Self, Self::Error> {
        if value.size > 8 {
            return Err(DisassemblerError::VarBitSizeExceeded(value.size, "u8"));
        }
        
        Ok(value.bits as u8)
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
