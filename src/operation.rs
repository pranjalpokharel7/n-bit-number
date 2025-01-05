use crate::{bigint::BIGINT, constants::BLOCK_DIVISION_FACTOR};
use std::cmp::min;

fn op_adc_u64(a: u64, b: u64, carry: &mut u64) -> u64 {
    let res = a + b + *carry;
    *carry = res / BLOCK_DIVISION_FACTOR;
    res % BLOCK_DIVISION_FACTOR
}

fn op_sbb_u64(a: u64, b: u64, borrow: &mut u64) -> u64 {
    return if a < b + *borrow {
        let res = a + BLOCK_DIVISION_FACTOR - b - *borrow;
        *borrow = 1;
        res
    } else {
        let res = a - b - *borrow;
        *borrow = 0;
        res
    };
}

pub fn op_add(lhs: &BIGINT, rhs: &BIGINT) -> Vec<u64> {
    // assume both lhs and rhs are of the same sign, handle sign logic elsewhere
    debug_assert_eq!(lhs.is_negative(), rhs.is_negative());

    let i = lhs.get_repr().len();
    let j = rhs.get_repr().len();
    let k = min(i, j);

    let mut result: Vec<u64> = Vec::new();
    let mut t = 0;
    let mut cin = 0;

    while t < k {
        result.push(op_adc_u64(lhs[t], rhs[t], &mut cin));
        t += 1;
    }

    while t < i {
        result.push(lhs[t] + cin);
        cin = 0;
        t += 1;
    }

    while t < j {
        result.push(rhs[t] + cin);
        cin = 0;
        t += 1;
    }

    result
}

pub fn op_sub(lhs: &BIGINT, rhs: &BIGINT) -> Vec<u64> {
    // assuming lhs > rhs for this to hold, handle all sign logic outside
    debug_assert!(lhs > rhs);

    let mut result: Vec<u64> = Vec::new();

    // a - b will always be possible given we handle sign negation
    // since a > b, a must have an equal or greater length to b
    let mut t = 0;
    let mut bout = 0;
    while t < rhs.get_repr().len() {
        result.push(op_sbb_u64(lhs[t], rhs[t], &mut bout));
        t += 1;
    }

    while t < lhs.get_repr().len() {
        result.push(lhs[t] - bout);
        bout = 0;
        t += 1;
    }

    result
}
