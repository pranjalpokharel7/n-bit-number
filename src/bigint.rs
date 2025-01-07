use crate::constants::DIGITS_PER_BLOCK;
use crate::operation::{op_add, op_sub};
use crate::utils::{big_n_str_to_vec, coalese_vector};
use std::ops::{Index, Sub};
use std::{
    fmt::{Debug, Display},
    ops::{Add, Neg},
};

#[derive(Debug, Clone)]
pub struct BIGINT {
    _signed: bool, // TODO: will handle this later
    _repr: Vec<u64>,
}

impl BIGINT {
    pub fn new(n: &str) -> Self {
        if n.is_empty() {
            panic!("Invalid number: expected numeric string, got empty.")
        }

        let (signed, repr) = if let Some(n_stripped) = n.strip_prefix("-") {
            (true, big_n_str_to_vec(n_stripped))
        } else {
            (false, big_n_str_to_vec(n))
        };

        Self {
            _signed: signed,
            _repr: Vec::from(coalese_vector(&repr)),
        }
    }

    // need to rename this function (?)
    pub fn new_sign_repr(sign: bool, repr: Vec<u64>) -> BIGINT {
        BIGINT {
            _signed: sign,
            _repr: Vec::from(coalese_vector(&repr)), // incurring memory initialization penalty for now
        }
    }

    pub fn get_repr(&self) -> &Vec<u64> {
        &self._repr
    }

    pub fn get_sign(&self) -> bool {
        self._signed
    }
}

impl Index<usize> for BIGINT {
    type Output = u64;

    fn index(&self, index: usize) -> &Self::Output {
        &self._repr[index]
    }
}

impl PartialEq for BIGINT {
    fn eq(&self, other: &Self) -> bool {
        self._signed == other._signed && self._repr == other._repr
    }
}

// sign handling logic courtesy of chat gpt :)
impl PartialOrd for BIGINT {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self._signed, other._signed) {
            (true, false) => Some(std::cmp::Ordering::Less), // negative < positive
            (false, true) => Some(std::cmp::Ordering::Greater), // positive > negative
            _ => {
                let cmp = self
                    ._repr
                    .iter()
                    .rev()
                    .partial_cmp(other._repr.iter().rev());

                if self._signed {
                    cmp.map(|ord| ord.reverse())
                } else {
                    cmp
                }
            }
        }
    }
}

impl Neg for BIGINT {
    type Output = BIGINT;

    fn neg(mut self) -> Self::Output {
        self._signed = !self._signed;
        self
    }
}

impl Sub<BIGINT> for BIGINT {
    type Output = BIGINT;

    fn sub(self, rhs: BIGINT) -> Self::Output {
        match (self._signed, rhs._signed) {
            (false, false) => op_sub(&self, &rhs),
            (false, true) => op_add(&self, &rhs),
            (true, false) => op_add(&self, &rhs).neg(),
            (true, true) => op_sub(&rhs, &self),
        }
    }
}

impl Add<BIGINT> for BIGINT {
    type Output = BIGINT;

    fn add(self, rhs: Self) -> Self::Output {
        match (self._signed, rhs._signed) {
            (false, false) => op_add(&self, &rhs),
            (false, true) => op_sub(&self, &rhs),
            (true, false) => op_sub(&rhs, &self),
            (true, true) => op_add(&self, &rhs).neg(),
        }
    }
}

impl Display for BIGINT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let k = self._repr.len();
        let mut repr_s: Vec<String> = Vec::new();

        // since we don't have to pad for the last block (unnecessary leading zeroes)
        repr_s.push(self._repr[k - 1].to_string());

        // iterate in reverse because the numbers are stored least significant block first
        for i in (0..k.saturating_sub(1)).rev() {
            let s = self._repr[i].to_string();
            repr_s.push(format!(
                "{}{}",
                "0".repeat((DIGITS_PER_BLOCK - s.len()) as usize),
                s
            ));
        }

        let sign = if self._signed { "-" } else { "" };

        write!(f, "{}{}", sign, repr_s.join(""))
    }
}
