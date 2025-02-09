use crate::constants::DIGITS_PER_BLOCK;
use crate::operation::{ op_add, op_sub };
use crate::utils::{ big_n_str_to_vec, coalesce_vector };
use std::cmp::Ordering;
use std::ops::{ Index, Shl, Shr, Sub };
use std::{ fmt::{ Debug, Display }, ops::{ Add, Neg, Deref } };

#[derive(Debug, Clone)]
pub struct BIGINT {
    _signed: bool,
    _repr: Vec<u64>,
}

// truth table for two variables x and y
macro_rules! tt_a_b_00_01_10_11 {
    ($x:expr, $y:expr, $z_00:expr, $z_01:expr, $z_10:expr, $z_11:expr) => {
        match ($x, $y) {
            (false, false) => $z_00,
            (false, true) => $z_01,
            (true, false) => $z_10,
            (true, true) => $z_11,
        }
    };
}

impl BIGINT {
    pub fn new(n: &str) -> Self {
        if n.is_empty() {
            panic!("Invalid number: expected numeric string, got empty.");
        }

        let (mut signed, _repr) = if let Some(n_stripped) = n.strip_prefix("-") {
            (true, big_n_str_to_vec(n_stripped))
        } else {
            (false, big_n_str_to_vec(n))
        };

        let repr = Vec::from(coalesce_vector(&_repr));

        // handle negative zero
        if repr == vec![0] {
            signed = false;
        }

        Self {
            _signed: signed,
            _repr: repr,
        }
    }

    // need to rename this function (?)
    pub fn from_repr(signed: bool, repr: Vec<u64>) -> BIGINT {
        let repr = Vec::from(coalesce_vector(&repr)); // incurring memory initialization penalty for now

        // handle negative zero
        let mut signed = signed;
        if repr == vec![0] {
            signed = false;
        }

        BIGINT {
            _signed: signed,
            _repr: repr,
        }
    }

    pub fn get_sign(&self) -> bool {
        self._signed
    }

    pub fn compare_magnitude(&self, other: &Self) -> Ordering {
        self._repr.iter().rev().cmp(other._repr.iter().rev())
    }
}

impl Deref for BIGINT {
    type Target = [u64];

    fn deref(&self) -> &Self::Target {
        &self._repr
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
        // handle negative zeroes in equality
        self._signed == other._signed && self._repr == other._repr
    }
}

impl Eq for BIGINT {}

impl PartialOrd for BIGINT {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        tt_a_b_00_01_10_11!(
            self._signed,
            other._signed,
            self._repr.iter().rev().partial_cmp(other._repr.iter().rev()),
            Some(std::cmp::Ordering::Greater),
            Some(std::cmp::Ordering::Less),
            self._repr
                .iter()
                .rev()
                .partial_cmp(other._repr.iter().rev())
                .map(|ord| ord.reverse())
        )
    }
}

impl Ord for BIGINT {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        tt_a_b_00_01_10_11!(
            self._signed,
            other._signed,
            self._repr.iter().rev().cmp(other._repr.iter().rev()),
            std::cmp::Ordering::Greater,
            std::cmp::Ordering::Less,
            self._repr.iter().rev().cmp(other._repr.iter().rev()).reverse()
        )
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
        tt_a_b_00_01_10_11!(
            self._signed,
            rhs._signed,
            op_sub(&self, &rhs),
            op_add(&self, &rhs),
            op_add(&self, &rhs).neg(),
            op_sub(&rhs, &self)
        )
    }
}

impl Add<BIGINT> for BIGINT {
    type Output = BIGINT;

    fn add(self, rhs: Self) -> Self::Output {
        tt_a_b_00_01_10_11!(
            self._signed,
            rhs._signed,
            op_add(&self, &rhs),
            op_sub(&self, &rhs),
            op_sub(&rhs, &self),
            op_add(&self, &rhs).neg()
        )
    }
}

impl Shl<u64> for &mut BIGINT {
    type Output = ();

    fn shl(self, rhs: u64) -> Self::Output {
        for n in self {
            *n = *n << rhs;
        }
    }
}

impl Shr<u64> for &mut BIGINT {
    type Output = ();

    fn shr(self, rhs: u64) -> Self::Output {
        for n in self {
            *n = *n >> rhs;
        }
    }
}

impl IntoIterator for BIGINT {
    type Item = u64;
    type IntoIter = std::vec::IntoIter<u64>;

    fn into_iter(self) -> Self::IntoIter {
        self._repr.into_iter()
    }
}

impl<'a> IntoIterator for &'a mut BIGINT {
    type Item = &'a mut u64;
    type IntoIter = std::slice::IterMut<'a, u64>;

    fn into_iter(self) -> Self::IntoIter {
        self._repr.iter_mut()
    }
}

impl<'a> IntoIterator for &'a BIGINT {
    type Item = &'a u64;
    type IntoIter = std::slice::Iter<'a, u64>;

    fn into_iter(self) -> Self::IntoIter {
        self._repr.iter()
    }
}

impl Display for BIGINT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let k = self._repr.len();
        let mut repr_s: Vec<String> = Vec::new();

        // since we don't have to pad for the last block (unnecessary leading zeroes)
        repr_s.push(self[k - 1].to_string());

        // iterate in reverse because the numbers are stored least significant block first
        for i in (0..k - 1).rev() {
            let s = self[i].to_string();
            repr_s.push(format!("{}{}", "0".repeat((DIGITS_PER_BLOCK - s.len()) as usize), s));
        }

        let sign = if self._signed { "-" } else { "" };

        write!(f, "{}{}", sign, repr_s.join(""))
    }
}
