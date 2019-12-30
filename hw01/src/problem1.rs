// problem1.rs

/// Functions are private (only available to this module) by default.
/// Use the `pub` keyword to mark this function as public.
pub fn sum(slice: &[i32]) -> i32 {
    let mut ret = 0;
    for i in slice {
        ret += i;
    }
    ret
}

pub fn dedup(vs: &Vec<i32>) -> Vec<i32> {
    let mut ret = Vec::new();
    for i in vs {
        if ret.contains(i) {
            continue;
        }
        ret.push(*i);
    }
    ret
}

pub fn filter(vs: &Vec<i32>, pred: &Fn(i32) -> bool) -> Vec<i32> {
    let mut ret = Vec::new();
    for i in vs {
        if pred(*i) {
            ret.push(*i);
        }
    }
    ret
}
