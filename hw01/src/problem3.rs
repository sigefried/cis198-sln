/// Find all prime numbers less than `n`.
/// For example, `sieve(7)` should return `[2, 3, 5]`
pub fn sieve(n: u32) -> Vec<u32> {
    let mut ret = Vec::new();
    let mut flag = Vec::new();

    for i in 2..n {
        if !flag.contains(&i) {
            ret.push(i);
            let mut j = i;
            while j < n {
                flag.push(j);
                j += i;
            }
        }
    }
    ret
}
