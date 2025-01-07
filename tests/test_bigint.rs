use n_bit_number::bigint;

#[cfg(test)]
mod tests {
    use super::*;
    use bigint::BIGINT;

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
    fn test_sorting() {
        let b1 = BIGINT::new("11000000000000000000");
        let b2 = BIGINT::new("-220000000000000000000");
        let b3 = BIGINT::new("12000000000000000000");
        let b4 = BIGINT::new("0");
        let b5 = BIGINT::new("-90000000000000000000");

        let mut vb = vec![b1.clone(), b2.clone(), b3.clone(), b4.clone(), b5.clone()];
        vb.sort();
        assert_eq!(vb, vec![b2, b5, b4, b1, b3]);
    }

    #[test]
    fn test_signed_zero() {
        let b0 = BIGINT::new("-0");
        assert_eq!(b0, BIGINT::new("0"));

        let b1 = BIGINT::new("1");
        let b2 = b1.clone() + b0.clone();
        assert_eq!(b2, b1);

        let b1 = BIGINT::new("-1");
        let b2 = b1.clone() + b0.clone();
        assert_eq!(b2, b1);
    }

    #[test]
    fn test_negation() {
        let b0 = BIGINT::new("0");
        let b1 = BIGINT::new("-1");
        let b2 = b0 - b1.clone();
        assert_eq!(b2, -b1);
    }

    #[test]
    fn test_coalesce() {
        let b1 = BIGINT::new("000000000000000000000000000000000000000000000000000000000000");
        assert_eq!(b1, BIGINT::new("0"));

        let b1 = BIGINT::new("000000000000000000000000000000000000000000000000000000000001");
        assert_eq!(b1, BIGINT::new("1"));
    }
}
