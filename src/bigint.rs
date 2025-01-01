use std::{
    cmp::min,
    fmt::{Debug, Display},
    ops::Add,
    str::FromStr,
};

trait BlockLimit {
    const LIMIT: usize;
}

impl BlockLimit for u64 {
    const LIMIT: usize = 18;
}

#[derive(Debug, Clone)]
pub struct BIGINT {
    pub value: String,
    _signed: bool,   // TODO: will handle this later
    _repr: Vec<u64>, // TODO: need to make this a template as well?
}

// maximum of u64 - 18446744073709551615
// max of i64 - 9223372036854775807
// take chunks of 18 length (to stay on the safe side)

// TODO: make this a generic container that can return u8, u16, u32, u64 or u128
// on second thought I don't see why I need this as a generic function hmm
fn big_n_str_to_vec<T: FromStr + BlockLimit>(n: &str) -> Vec<T> {
    let s_len = n.len();
    let mut j = s_len;
    let mut i = j.saturating_sub(T::LIMIT);
    let mut result: Vec<T> = Vec::new();
    while j > 0 {
        let p_res: Result<T, T::Err> = n[i..j].parse();
        match p_res {
            Ok(p) => {
                result.push(p);
                j = i;
                i = i.saturating_sub(18);
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
            value: n.to_owned(),
            _signed: signed,
            _repr: repr,
        }
    }
}

impl Add<BIGINT> for BIGINT {
    type Output = BIGINT;

    fn add(self, rhs: Self) -> Self::Output {
        let i = self._repr.len();
        let j = rhs._repr.len();
        let k = min(i, j);
        let mut repr: Vec<u64> = Vec::new();
        let factor = 10_u64.pow(18);

        let mut cin = 0;
        let mut t = 0;
        while t < k {
            let s = self._repr[t] + rhs._repr[t] + cin;
            cin = s / factor;
            repr.push(s % factor);
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

        let mut value = String::new();
        for &n in &repr {
            value += &n.to_string();
        }

        BIGINT {
            value,
            _signed: false,
            _repr: repr,
        }
    }
}

impl Display for BIGINT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
