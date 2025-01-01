pub mod bigint;

#[cfg(test)]
mod tests {
    use super::*;
    use bigint::BIGINT;

    #[test]
    fn test_increment() {
        let b1 = BIGINT::new("10");
        let b2 = BIGINT::new("1");
        let b3 = b1 + b2;
        assert_eq!(b3.value, "11");
    }

    #[test]
    fn test_identity() {
        let identity = BIGINT::new("0");
        
        // additive identity added to itself should be result the same
        let b2 = BIGINT::new("0");
        let b3 = identity.clone() + b2.clone();
        assert_eq!(b3.value, b2.value);

        // additive identity added to any n should result the sum being n
        let b4 = BIGINT::new("1234");
        let b5 = identity + b4.clone();
        assert_eq!(b5.value, b4.value);
    }
}
