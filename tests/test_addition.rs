use n_bit_number::bigint;

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
    fn test_addition() {
        // adding on different signs
        let b1 = BIGINT::new("100000000000000000000");
        let b2 = BIGINT::new("100000000000000000000");
        let b3 = b1 + b2;
        assert_eq!(b3, BIGINT::new("200000000000000000000"));

        let b1 = BIGINT::new("-100000000000000000000");
        let b2 = BIGINT::new("-100000000000000000000");
        let b3 = b1 + b2;
        assert_eq!(b3, BIGINT::new("-200000000000000000000"));

        let b1 = BIGINT::new("100000000000000000000");
        let b2 = BIGINT::new("-100000000000000000000");
        let b3 = b1 + b2;
        assert_eq!(b3, BIGINT::new("0"));

        let b1 = BIGINT::new("-100000000000000000000");
        let b2 = BIGINT::new("100000000000000000000");
        let b3 = b1 + b2;
        assert_eq!(b3, BIGINT::new("0"));
    }
}
