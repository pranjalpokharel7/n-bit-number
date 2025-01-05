use std::{cmp::min, fmt::{Debug, Display}, ops::Add, str::FromStr};

// maximum number that can be held by u64 - 18446744073709551615 - 20 digits
// take chunks of 18 length (to stay on the safe side)
static DIGITS_PER_BLOCK: usize = 18;
static BLOCK_DIVISION_FACTOR: u64 = 10_u64.pow(DIGITS_PER_BLOCK as u32);

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
}

impl PartialEq for BIGINT {
    fn eq(&self, other: &Self) -> bool {
        let len = self._repr.len();
        if len != other._repr.len() {
            return false;
        }

        for i in 0..len {
            if self._repr[i] != other._repr[i] {
                return false;
            }
        }

        return true;
    }
}

impl Add<BIGINT> for BIGINT {
    type Output = BIGINT;

    fn add(self, rhs: Self) -> Self::Output {
        let i = self._repr.len();
        let j = rhs._repr.len();
        let k = min(i, j);
        let mut repr: Vec<u64> = Vec::new();

        let mut cin = 0;
        let mut t = 0;
        while t < k {
            let s = self._repr[t] + rhs._repr[t] + cin;
            cin = s / BLOCK_DIVISION_FACTOR;
            repr.push(s % BLOCK_DIVISION_FACTOR);
            t += 1;
        }

        while t < i {
            repr.push(self._repr[t] + cin);
            cin = 0;
            t += 1;
        }

        while t < j {
            repr.push(rhs._repr[t] + cin);
            cin = 0;
            t += 1;
        }

        BIGINT {
            _signed: false,
            _repr: repr,
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
        for i in (0..=k - 2).rev() {
            let s = self._repr[i].to_string();
            repr_s.push(format!(
                "{}{}",
                "0".repeat((DIGITS_PER_BLOCK - s.len()) as usize),
                s
            ));
        }
        
        write!(f, "{}", repr_s.join(""))
    }
}
