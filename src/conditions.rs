use crate::Error;

#[derive(Debug, PartialEq)]
#[repr(u8)]
pub enum Condition {
    EQ = 0,
    NE = 1,
    LT = 4,
    GE = 5,
    LTU = 6,
    GEU = 7,
}

impl TryFrom<u8> for Condition {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Condition::EQ),
            1 => Ok(Condition::NE),
            4 => Ok(Condition::LT),
            5 => Ok(Condition::GE),
            6 => Ok(Condition::LTU),
            7 => Ok(Condition::GEU),
            _ => Err(Error::InvalidCondition),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_from() {
        assert_eq!(Condition::try_from(0), Ok(Condition::EQ));
        assert_eq!(Condition::try_from(1), Ok(Condition::NE));
        assert_eq!(Condition::try_from(4), Ok(Condition::LT));
        assert_eq!(Condition::try_from(5), Ok(Condition::GE));
        assert_eq!(Condition::try_from(6), Ok(Condition::LTU));
        assert_eq!(Condition::try_from(7), Ok(Condition::GEU));
        assert_eq!(Condition::try_from(8), Err(Error::InvalidCondition));
    }
}