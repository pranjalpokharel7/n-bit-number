use crate::constants::DIGITS_PER_BLOCK;
use std::str::FromStr;

pub fn big_n_str_to_vec<T: FromStr>(n: &str) -> Vec<T> {
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

// iterate in reverse until no zeroes are encountered - remove leading zeroes
// [0,0,0] should be stored as [0] since [0] and [0,0,0] are equivalent
pub fn coalesce_vector(v: &Vec<u64>) -> &[u64] {
    for i in (0..v.len()).rev() {
        if v[i] != 0 {
            return &v[0..=i];
        }
    };
    return &v[0..=0];
}
