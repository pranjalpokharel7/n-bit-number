use crate::constants::DIGITS_PER_BLOCK;
use crate::operation::{op_add, op_sub};
use std::ops::{Index, Sub};
use std::{
    fmt::{Debug, Display},
    ops::{Add, Neg},
    str::FromStr,
};

#[derive(Debug, Clone)]
pub struct BIGINT {
    _signed: bool, // TODO: will handle this later
    _repr: Vec<u64>,
}

fn big_n_str_to_vec<T: FromStr>(n: &str) -> Vec<T> {
    let s_len = n.len();
    let mut j = s_len;
    let mut i = j.saturating_sub(DIGITS_PER_BLOCK);
    let mut result: Vec<T> = Vec::new();
    while j > 0 {
        let p_res: Result<T, T::Err> = n[i..j].parse();
        match p_res {
            Ok(p) => {
                result.push(p);
                j = i;
                i = i.saturating_sub(DIGITS_PER_BLOCK);
            }
            Err(_) => panic!("Couldn't parse number - invalid literal"), // TODO: handle where exactly error occurs
        }
    }
    result
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
            _repr: repr,
        }
    }

    pub fn get_repr(&self) -> &Vec<u64> {
        &self._repr
    }

    pub fn is_negative(&self) -> bool {
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
        if self._signed || rhs._signed {
            // TODO: handle this
            unimplemented!();
        }

        let (signed, a, b) = if self > rhs {
            (false, &self, &rhs)
        } else {
            (true, &rhs, &self)
        };

        // a - b will always be possible given we handle sign negation
        // since a > b, a must have an equal or greater length to b
        let result = op_sub(&a, &b);

        Self {
            _signed: signed,
            _repr: result,
        }
    }
}

impl Add<BIGINT> for BIGINT {
    type Output = BIGINT;

    // TODO: abstract this to another method that is sign agnostic
    fn add(self, rhs: Self) -> Self::Output {
        // same sign i.e. both true or both false indicates addition (XNOR)
        let sign_similarity = !(self._signed ^ rhs._signed);
        if !sign_similarity {
            // TODO: numbers with different signs not handled
            unimplemented!();
        }

        let result: Vec<u64> = op_add(&self, &rhs);

        BIGINT {
            _signed: self._signed,
            _repr: result,
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
