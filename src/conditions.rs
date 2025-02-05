use crate::Error;

#[derive(Debug, PartialEq)]
#[repr(u8)]
pub enum Condition {
    Equal = 0,
    NotEqual = 1,
    LessThan = 2,
    LessThanUnsigned = 3,
    GreaterEqual = 4,
    GreaterEqualUnsigned = 5,
}

impl TryFrom<u8> for Condition {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Condition::Equal),
            1 => Ok(Condition::NotEqual),
            2 => Ok(Condition::LessThan),
            3 => Ok(Condition::LessThanUnsigned),
            4 => Ok(Condition::GreaterEqual),
            5 => Ok(Condition::GreaterEqualUnsigned),
            _ => Err(Error::InvalidCondition),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_from() {
        assert_eq!(Condition::try_from(0), Ok(Condition::Equal));
        assert_eq!(Condition::try_from(1), Ok(Condition::NotEqual));
        assert_eq!(Condition::try_from(2), Ok(Condition::LessThan));
        assert_eq!(Condition::try_from(3), Ok(Condition::LessThanUnsigned));
        assert_eq!(Condition::try_from(4), Ok(Condition::GreaterEqual));
        assert_eq!(Condition::try_from(5), Ok(Condition::GreaterEqualUnsigned));
        assert_eq!(Condition::try_from(6), Err(Error::InvalidCondition));
    }
}