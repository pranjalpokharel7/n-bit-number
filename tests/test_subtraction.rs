#[cfg(test)]
mod tests {
    use n_bit_number::bigint::BIGINT;

    #[test]
    fn test_subtraction_decrement() {
        let b1 = BIGINT::new("10000000000000000000");
        let b2 = BIGINT::new("1");
        let b3 = b1 - b2;
        assert_eq!(b3, BIGINT::new("9999999999999999999"));
    }


    #[test]
    fn test_subtraction() {
        let b1 = BIGINT::new("11000000000000000000");
        let b2 = BIGINT::new("1000000000000000000");
        let b3 = b1 - b2;
        assert_eq!(b3, BIGINT::new("10000000000000000000"));

        let b1 = BIGINT::new("10000000000000000000");
        let b2 = BIGINT::new("1");
        let b3 = b2 - b1;
        assert_eq!(b3, BIGINT::new("-9999999999999999999"));

        let b1 = BIGINT::new("100000000000000000000");
        let b2 = BIGINT::new("-100000000000000000000");
        let b3 = b1 - b2;
        assert_eq!(b3, BIGINT::new("200000000000000000000"));

        let b1 = BIGINT::new("100000000000000000000");
        let b2 = BIGINT::new("-100000000000000000000");
        let b3 = b2 - b1;
        assert_eq!(b3, BIGINT::new("-200000000000000000000"));

        let b1 = BIGINT::new("-100000000000000000000");
        let b2 = BIGINT::new("-100000000000000000000");
        let b3 = b2 - b1;
        assert_eq!(b3, BIGINT::new("0"));
    }
}
