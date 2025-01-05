use crate::constants::BLOCK_DIVISION_FACTOR;

pub fn adc(a: &u64, b: &u64, carry: &mut u64) -> u64 {
    let res = a + b + *carry;
    *carry = res / BLOCK_DIVISION_FACTOR;
    res % BLOCK_DIVISION_FACTOR
}
