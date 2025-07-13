use std::error::Error;

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<Self, CreationError> {
        if value < 0 {
            return Err(CreationError::Negative);
        }

        if value == 0 {
            return Err(CreationError::Zero);
        }
        let res = Self(value as u64);
        Ok(res)
    }
}

fn main() {
    // You can optionally experiment here.
    let x = PositiveNonzeroInteger::new(10);
    format!("{:?}", x);
    let x = PositiveNonzeroInteger::new(-10);
    format!("{:?}", x);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        assert_eq!(
            PositiveNonzeroInteger::new(10),
            Ok(PositiveNonzeroInteger(10)),
        );
        assert_eq!(
            PositiveNonzeroInteger::new(-10),
            Err(CreationError::Negative),
        );
        assert_eq!(PositiveNonzeroInteger::new(0), Err(CreationError::Zero));
    }
}
