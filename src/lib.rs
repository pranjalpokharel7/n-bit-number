pub mod bigint;
pub mod constants;
pub mod operation;

#[cfg(test)]
mod tests {
    use super::*;
    use bigint::BIGINT;

    #[test]
    fn test_increment() {
        let b1 = BIGINT::new("10");
        let b2 = BIGINT::new("1");
        let b3 = b1 + b2;
        assert_eq!(b3, BIGINT::new("11"));
    }

    #[test]
    fn test_identity() {
        let identity = BIGINT::new("0");

        // additive identity added to itself should be result the same
        let b2 = BIGINT::new("0");
        let b3 = identity.clone() + b2.clone();
        assert_eq!(b3, b2);

        // additive identity added to any n should result the sum being n
        let b4 = BIGINT::new("1234");
        let b5 = identity + b4.clone();
        assert_eq!(b5, b4);
    }

    #[test]
    fn test_to_string() {
        // zero padding test
        let b2 = BIGINT::new("1000000000000000000");
        assert_eq!(b2.to_string(), "1000000000000000000");

        // sign test
        let b3 = BIGINT::new("-1");
        assert_eq!(b3.to_string(), "-1");
    }

    #[test]
    fn test_signed_operation() {
        // adding two negative numbers
        let b1 = BIGINT::new("-1");
        let b2 = BIGINT::new("-1");
        let b3 = b1 + b2;
        assert_eq!(b3, BIGINT::new("-2"));

        // subtractions on different signs
        let b1 = BIGINT::new("100000000000000000000");
        let b2 = BIGINT::new("-100000000000000000000");
        let b3 = b1 - b2;
        assert_eq!(b3, BIGINT::new("200000000000000000000"));

        let b1 = BIGINT::new("100000000000000000000");
        let b2 = BIGINT::new("-100000000000000000000");
        let b3 = b2 - b1;
        assert_eq!(b3, BIGINT::new("-200000000000000000000"));
    }

    #[test]
    fn test_comparision() {
        let b1 = BIGINT::new("11000000000000000000");
        let b2 = BIGINT::new("12000000000000000000");
        let b3 = BIGINT::new("9000000000000000000");
        let b4 = BIGINT::new("-12000000000000000000");
        let b5 = BIGINT::new("-9000000000000000000");

        assert!(b2 > b1);
        assert!(b1 > b3);
        assert!(b1 > b4);
        assert!(b5 > b4);
    }

    #[test]
    fn test_subtraction() {
        let b1 = BIGINT::new("11000000000000000000");
        let b2 = BIGINT::new("1000000000000000000");
        let b3 = b1 - b2;
        assert_eq!(b3, BIGINT::new("10000000000000000000"));

        let b1 = BIGINT::new("10000000000000000000");
        let b2 = BIGINT::new("1");
        let b3 = b1 - b2;
        assert_eq!(b3, BIGINT::new("9999999999999999999"));

        let b1 = BIGINT::new("10000000000000000000");
        let b2 = BIGINT::new("1");
        let b3 = b2 - b1;
        assert_eq!(b3.get_sign(), true);
        assert_eq!(b3, BIGINT::new("-9999999999999999999"));
    }
}
