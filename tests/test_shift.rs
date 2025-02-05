#[cfg(test)]
mod tests {
    use n_bit_number::bigint::BIGINT;

    #[test]
    fn test_shift_left() {
        let mut b1 = BIGINT::new("1");
        &mut b1 << 1;
        assert_eq!(b1, BIGINT::new("2"));

        let mut b2 = BIGINT::from_repr(false, vec![1, 2, 3]);
        &mut b2 << 1;
        assert_eq!(b2, BIGINT::from_repr(false, vec![2, 4, 6]));
    }

    #[test]
    fn test_shift_right() {
        let mut b1 = BIGINT::new("2");
        &mut b1 >> 1_u64;
        assert_eq!(b1, BIGINT::new("1"));

        let mut b2 = BIGINT::from_repr(false, vec![2, 4, 6]);
        &mut b2 >> 1;
        assert_eq!(b2, BIGINT::from_repr(false, vec![1, 2, 3]));
    }
}
