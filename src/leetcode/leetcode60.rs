struct Solution;

impl Solution {
    pub fn get_permutation(n: i32, k: i32) -> String {
        let n = n as usize;
        let mut k = k as usize;
        let mut res = Vec::with_capacity(n);
        let mut v = (1..=n).collect::<Vec<_>>();
        let factorial = vec![1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880];

        k -= 1;
        let mut i = n;
        loop {
            let r = k % factorial[i - 1];
            let t = k / factorial[i - 1];
            k = r;
            res.push(v[t] as u8 + '0' as u8);
            v.remove(t);
            if i > 1 {
                i -= 1;
            } else {
                break;
            }
        }

        unsafe { String::from_utf8_unchecked(res) }
    }
}

#[test]
fn leetcode60_t1() {
    let r = Solution::get_permutation(3, 2);
    println!("{}", r);
}
