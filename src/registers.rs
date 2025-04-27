use std::fmt;
use crate::DisassemblerError;

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum Register {
    x0 = 0,
    x1 = 1,
    x2 = 2,
    x3 = 3,
    x4 = 4,
    x5 = 5,
    x6 = 6,
    x7 = 7,
    x8 = 8,
    x9 = 9,
    x10 = 10,
    x11 = 11,
    x12 = 12,
    x13 = 13,
    x14 = 14,
    x15 = 15,
    x16 = 16,
    x17 = 17,
    x18 = 18,
    x19 = 19,
    x20 = 20,
    x21 = 21,
    x22 = 22,
    x23 = 23,
    x24 = 24,
    x25 = 25,
    x26 = 26,
    x27 = 27,
    x28 = 28,
    x29 = 29,
    x30 = 30,
    x31 = 31,
}

impl TryFrom<u8> for Register {
    type Error = DisassemblerError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Register::x0),
            1 => Ok(Register::x1),
            2 => Ok(Register::x2),
            3 => Ok(Register::x3),
            4 => Ok(Register::x4),
            5 => Ok(Register::x5),
            6 => Ok(Register::x6),
            7 => Ok(Register::x7),
            8 => Ok(Register::x8),
            9 => Ok(Register::x9),
            10 => Ok(Register::x10),
            11 => Ok(Register::x11),
            12 => Ok(Register::x12),
            13 => Ok(Register::x13),
            14 => Ok(Register::x14),
            15 => Ok(Register::x15),
            16 => Ok(Register::x16),
            17 => Ok(Register::x17),
            18 => Ok(Register::x18),
            19 => Ok(Register::x19),
            20 => Ok(Register::x20),
            21 => Ok(Register::x21),
            22 => Ok(Register::x22),
            23 => Ok(Register::x23),
            24 => Ok(Register::x24),
            25 => Ok(Register::x25),
            26 => Ok(Register::x26),
            27 => Ok(Register::x27),
            28 => Ok(Register::x28),
            29 => Ok(Register::x29),
            30 => Ok(Register::x30),
            31 => Ok(Register::x31),
            _ => Err(DisassemblerError::InvalidRegister(value)),
        }
    }
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x{}", *self as u8)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_from_register() {
        assert_eq!(0.try_into(), Ok(Register::x0));
        assert_eq!(1.try_into(), Ok(Register::x1));
        assert_eq!(2.try_into(), Ok(Register::x2));
        assert_eq!(3.try_into(), Ok(Register::x3));
        assert_eq!(4.try_into(), Ok(Register::x4));
        assert_eq!(5.try_into(), Ok(Register::x5));
        assert_eq!(6.try_into(), Ok(Register::x6));
        assert_eq!(7.try_into(), Ok(Register::x7));
        assert_eq!(8.try_into(), Ok(Register::x8));
        assert_eq!(9.try_into(), Ok(Register::x9));
        assert_eq!(10.try_into(), Ok(Register::x10));
        assert_eq!(11.try_into(), Ok(Register::x11));
        assert_eq!(12.try_into(), Ok(Register::x12));
        assert_eq!(13.try_into(), Ok(Register::x13));
        assert_eq!(14.try_into(), Ok(Register::x14));
        assert_eq!(15.try_into(), Ok(Register::x15));
        assert_eq!(16.try_into(), Ok(Register::x16));
        assert_eq!(17.try_into(), Ok(Register::x17));
        assert_eq!(18.try_into(), Ok(Register::x18));
        assert_eq!(19.try_into(), Ok(Register::x19));
        assert_eq!(20.try_into(), Ok(Register::x20));
        assert_eq!(21.try_into(), Ok(Register::x21));
        assert_eq!(22.try_into(), Ok(Register::x22));
        assert_eq!(23.try_into(), Ok(Register::x23));
        assert_eq!(24.try_into(), Ok(Register::x24));
        assert_eq!(25.try_into(), Ok(Register::x25));
        assert_eq!(26.try_into(), Ok(Register::x26));
        assert_eq!(27.try_into(), Ok(Register::x27));
        assert_eq!(28.try_into(), Ok(Register::x28));
        assert_eq!(29.try_into(), Ok(Register::x29));
        assert_eq!(30.try_into(), Ok(Register::x30));
        assert_eq!(31.try_into(), Ok(Register::x31));
    }
}