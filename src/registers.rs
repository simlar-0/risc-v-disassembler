use crate::DisassemblerError;
use std::fmt;

pub trait Register {
    fn as_u8(&self) -> u8;
    fn from_u8(value: u8) -> Result<Self, DisassemblerError>
    where
        Self: Sized;
    fn as_string(&self) -> String;
}

impl Register for NumberedRegister {
    fn as_u8(&self) -> u8 {
        *self as u8
    }

    fn from_u8(value: u8) -> Result<Self, DisassemblerError> {
        NumberedRegister::try_from(value)
    }

    fn as_string(&self) -> String {
        format!("x{}", *self as u8)
    }
}

impl Register for ABIRegister {
    fn as_u8(&self) -> u8 {
        *self as u8
    }

    fn from_u8(value: u8) -> Result<Self, DisassemblerError> {
        ABIRegister::try_from(value)
    }

    fn as_string(&self) -> String {
        match self {
            ABIRegister::zero => "zero".to_string(),
            ABIRegister::ra => "ra".to_string(),
            ABIRegister::sp => "sp".to_string(),
            ABIRegister::gp => "gp".to_string(),
            ABIRegister::tp => "tp".to_string(),
            ABIRegister::t0 => "t0".to_string(),
            ABIRegister::t1 => "t1".to_string(),
            ABIRegister::t2 => "t2".to_string(),
            ABIRegister::s0 => "s0".to_string(),
            ABIRegister::s1 => "s1".to_string(),
            ABIRegister::a0 => "a0".to_string(),
            ABIRegister::a1 => "a1".to_string(),
            ABIRegister::a2 => "a2".to_string(),
            ABIRegister::a3 => "a3".to_string(),
            ABIRegister::a4 => "a4".to_string(),
            ABIRegister::a5 => "a5".to_string(),
            ABIRegister::a6 => "a6".to_string(),
            ABIRegister::a7 => "a7".to_string(),
            ABIRegister::s2 => "s2".to_string(),
            ABIRegister::s3 => "s3".to_string(),
            ABIRegister::s4 => "s4".to_string(),
            ABIRegister::s5 => "s5".to_string(),
            ABIRegister::s6 => "s6".to_string(),
            ABIRegister::s7 => "s7".to_string(),
            ABIRegister::s8 => "s8".to_string(),
            ABIRegister::s9 => "s9".to_string(),
            ABIRegister::s10 => "s10".to_string(),
            ABIRegister::s11 => "s11".to_string(),
            ABIRegister::t3 => "t3".to_string(),
            ABIRegister::t4 => "t4".to_string(),
            ABIRegister::t5 => "t5".to_string(),
            ABIRegister::t6 => "t6".to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum NumberedRegister {
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

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum ABIRegister {
    zero = 0,
    ra = 1,
    sp = 2,
    gp = 3,
    tp = 4,
    t0 = 5,
    t1 = 6,
    t2 = 7,
    s0 = 8,
    s1 = 9,
    a0 = 10,
    a1 = 11,
    a2 = 12,
    a3 = 13,
    a4 = 14,
    a5 = 15,
    a6 = 16,
    a7 = 17,
    s2 = 18,
    s3 = 19,
    s4 = 20,
    s5 = 21,
    s6 = 22,
    s7 = 23,
    s8 = 24,
    s9 = 25,
    s10 = 26,
    s11 = 27,
    t3 = 28,
    t4 = 29,
    t5 = 30,
    t6 = 31,
}

impl TryFrom<u8> for NumberedRegister {
    type Error = DisassemblerError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(NumberedRegister::x0),
            1 => Ok(NumberedRegister::x1),
            2 => Ok(NumberedRegister::x2),
            3 => Ok(NumberedRegister::x3),
            4 => Ok(NumberedRegister::x4),
            5 => Ok(NumberedRegister::x5),
            6 => Ok(NumberedRegister::x6),
            7 => Ok(NumberedRegister::x7),
            8 => Ok(NumberedRegister::x8),
            9 => Ok(NumberedRegister::x9),
            10 => Ok(NumberedRegister::x10),
            11 => Ok(NumberedRegister::x11),
            12 => Ok(NumberedRegister::x12),
            13 => Ok(NumberedRegister::x13),
            14 => Ok(NumberedRegister::x14),
            15 => Ok(NumberedRegister::x15),
            16 => Ok(NumberedRegister::x16),
            17 => Ok(NumberedRegister::x17),
            18 => Ok(NumberedRegister::x18),
            19 => Ok(NumberedRegister::x19),
            20 => Ok(NumberedRegister::x20),
            21 => Ok(NumberedRegister::x21),
            22 => Ok(NumberedRegister::x22),
            23 => Ok(NumberedRegister::x23),
            24 => Ok(NumberedRegister::x24),
            25 => Ok(NumberedRegister::x25),
            26 => Ok(NumberedRegister::x26),
            27 => Ok(NumberedRegister::x27),
            28 => Ok(NumberedRegister::x28),
            29 => Ok(NumberedRegister::x29),
            30 => Ok(NumberedRegister::x30),
            31 => Ok(NumberedRegister::x31),
            _ => Err(DisassemblerError::InvalidRegister(value)),
        }
    }
}

impl TryFrom<u8> for ABIRegister {
    type Error = DisassemblerError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ABIRegister::zero),
            1 => Ok(ABIRegister::ra),
            2 => Ok(ABIRegister::sp),
            3 => Ok(ABIRegister::gp),
            4 => Ok(ABIRegister::tp),
            5 => Ok(ABIRegister::t0),
            6 => Ok(ABIRegister::t1),
            7 => Ok(ABIRegister::t2),
            8 => Ok(ABIRegister::s0),
            9 => Ok(ABIRegister::s1),
            10 => Ok(ABIRegister::a0),
            11 => Ok(ABIRegister::a1),
            12 => Ok(ABIRegister::a2),
            13 => Ok(ABIRegister::a3),
            14 => Ok(ABIRegister::a4),
            15 => Ok(ABIRegister::a5),
            16 => Ok(ABIRegister::a6),
            17 => Ok(ABIRegister::a7),
            18 => Ok(ABIRegister::s2),
            19 => Ok(ABIRegister::s3),
            20 => Ok(ABIRegister::s4),
            21 => Ok(ABIRegister::s5),
            22 => Ok(ABIRegister::s6),
            23 => Ok(ABIRegister::s7),
            24 => Ok(ABIRegister::s8),
            25 => Ok(ABIRegister::s9),
            26 => Ok(ABIRegister::s10),
            27 => Ok(ABIRegister::s11),
            28 => Ok(ABIRegister::t3),
            29 => Ok(ABIRegister::t4),
            30 => Ok(ABIRegister::t5),
            31 => Ok(ABIRegister::t6),
            _ => Err(DisassemblerError::InvalidRegister(value)),
        }
    }
}

impl fmt::Display for NumberedRegister {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x{}", *self as u8)
    }
}

impl fmt::Display for ABIRegister {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ABIRegister::zero => write!(f, "zero"),
            ABIRegister::ra => write!(f, "ra"),
            ABIRegister::sp => write!(f, "sp"),
            ABIRegister::gp => write!(f, "gp"),
            ABIRegister::tp => write!(f, "tp"),
            ABIRegister::t0 => write!(f, "t0"),
            ABIRegister::t1 => write!(f, "t1"),
            ABIRegister::t2 => write!(f, "t2"),
            ABIRegister::s0 => write!(f, "s0"),
            ABIRegister::s1 => write!(f, "s1"),
            ABIRegister::a0 => write!(f, "a0"),
            ABIRegister::a1 => write!(f, "a1"),
            ABIRegister::a2 => write!(f, "a2"),
            ABIRegister::a3 => write!(f, "a3"),
            ABIRegister::a4 => write!(f, "a4"),
            ABIRegister::a5 => write!(f, "a5"),
            ABIRegister::a6 => write!(f, "a6"),
            ABIRegister::a7 => write!(f, "a7"),
            ABIRegister::s2 => write!(f, "s2"),
            ABIRegister::s3 => write!(f, "s3"),
            ABIRegister::s4 => write!(f, "s4"),
            ABIRegister::s5 => write!(f, "s5"),
            ABIRegister::s6 => write!(f, "s6"),
            ABIRegister::s7 => write!(f, "s7"),
            ABIRegister::s8 => write!(f, "s8"),
            ABIRegister::s9 => write!(f, "s9"),
            ABIRegister::s10 => write!(f, "s10"),
            ABIRegister::s11 => write!(f, "s11"),
            ABIRegister::t3 => write!(f, "t3"),
            ABIRegister::t4 => write!(f, "t4"),
            ABIRegister::t5 => write!(f, "t5"),
            ABIRegister::t6 => write!(f, "t6"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_from_register() {
        assert_eq!(0.try_into(), Ok(NumberedRegister::x0));
        assert_eq!(1.try_into(), Ok(NumberedRegister::x1));
        assert_eq!(2.try_into(), Ok(NumberedRegister::x2));
        assert_eq!(3.try_into(), Ok(NumberedRegister::x3));
        assert_eq!(4.try_into(), Ok(NumberedRegister::x4));
        assert_eq!(5.try_into(), Ok(NumberedRegister::x5));
        assert_eq!(6.try_into(), Ok(NumberedRegister::x6));
        assert_eq!(7.try_into(), Ok(NumberedRegister::x7));
        assert_eq!(8.try_into(), Ok(NumberedRegister::x8));
        assert_eq!(9.try_into(), Ok(NumberedRegister::x9));
        assert_eq!(10.try_into(), Ok(NumberedRegister::x10));
        assert_eq!(11.try_into(), Ok(NumberedRegister::x11));
        assert_eq!(12.try_into(), Ok(NumberedRegister::x12));
        assert_eq!(13.try_into(), Ok(NumberedRegister::x13));
        assert_eq!(14.try_into(), Ok(NumberedRegister::x14));
        assert_eq!(15.try_into(), Ok(NumberedRegister::x15));
        assert_eq!(16.try_into(), Ok(NumberedRegister::x16));
        assert_eq!(17.try_into(), Ok(NumberedRegister::x17));
        assert_eq!(18.try_into(), Ok(NumberedRegister::x18));
        assert_eq!(19.try_into(), Ok(NumberedRegister::x19));
        assert_eq!(20.try_into(), Ok(NumberedRegister::x20));
        assert_eq!(21.try_into(), Ok(NumberedRegister::x21));
        assert_eq!(22.try_into(), Ok(NumberedRegister::x22));
        assert_eq!(23.try_into(), Ok(NumberedRegister::x23));
        assert_eq!(24.try_into(), Ok(NumberedRegister::x24));
        assert_eq!(25.try_into(), Ok(NumberedRegister::x25));
        assert_eq!(26.try_into(), Ok(NumberedRegister::x26));
        assert_eq!(27.try_into(), Ok(NumberedRegister::x27));
        assert_eq!(28.try_into(), Ok(NumberedRegister::x28));
        assert_eq!(29.try_into(), Ok(NumberedRegister::x29));
        assert_eq!(30.try_into(), Ok(NumberedRegister::x30));
        assert_eq!(31.try_into(), Ok(NumberedRegister::x31));
    }

    #[test]
    fn test_try_from_abi_register() {
        assert_eq!(0.try_into(), Ok(ABIRegister::zero));
        assert_eq!(1.try_into(), Ok(ABIRegister::ra));
        assert_eq!(2.try_into(), Ok(ABIRegister::sp));
        assert_eq!(3.try_into(), Ok(ABIRegister::gp));
        assert_eq!(4.try_into(), Ok(ABIRegister::tp));
        assert_eq!(5.try_into(), Ok(ABIRegister::t0));
        assert_eq!(6.try_into(), Ok(ABIRegister::t1));
        assert_eq!(7.try_into(), Ok(ABIRegister::t2));
        assert_eq!(8.try_into(), Ok(ABIRegister::s0));
        assert_eq!(9.try_into(), Ok(ABIRegister::s1));
        assert_eq!(10.try_into(), Ok(ABIRegister::a0));
        assert_eq!(11.try_into(), Ok(ABIRegister::a1));
        assert_eq!(12.try_into(), Ok(ABIRegister::a2));
        assert_eq!(13.try_into(), Ok(ABIRegister::a3));
        assert_eq!(14.try_into(), Ok(ABIRegister::a4));
        assert_eq!(15.try_into(), Ok(ABIRegister::a5));
        assert_eq!(16.try_into(), Ok(ABIRegister::a6));
        assert_eq!(17.try_into(), Ok(ABIRegister::a7));
        assert_eq!(18.try_into(), Ok(ABIRegister::s2));
        assert_eq!(19.try_into(), Ok(ABIRegister::s3));
        assert_eq!(20.try_into(), Ok(ABIRegister::s4));
        assert_eq!(21.try_into(), Ok(ABIRegister::s5));
        assert_eq!(22.try_into(), Ok(ABIRegister::s6));
        assert_eq!(23.try_into(), Ok(ABIRegister::s7));
        assert_eq!(24.try_into(), Ok(ABIRegister::s8));
        assert_eq!(25.try_into(), Ok(ABIRegister::s9));
        assert_eq!(26.try_into(), Ok(ABIRegister::s10));
        assert_eq!(27.try_into(), Ok(ABIRegister::s11));
        assert_eq!(28.try_into(), Ok(ABIRegister::t3));
        assert_eq!(29.try_into(), Ok(ABIRegister::t4));
        assert_eq!(30.try_into(), Ok(ABIRegister::t5));
        assert_eq!(31.try_into(), Ok(ABIRegister::t6));
    }
}
