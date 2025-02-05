// maximum number that can be held by u64 - 18446744073709551615 - 20 digits
// take chunks of 18 length - this allows us to be a bit more flexible with overflow checks
pub(crate) static DIGITS_PER_BLOCK: usize = 18;
pub(crate) static BLOCK_DIVISION_FACTOR: u64 = 10_u64.pow(DIGITS_PER_BLOCK as u32);
