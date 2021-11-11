struct Solution;

impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let mut res = vec![];
        let mut path = vec![];
        Self::backtrace(0, &s, &mut path, &mut res);
        res
    }

    fn backtrace<'a>(left: usize, s: &'a String, path: &mut Vec<&'a str>, res: &mut Vec<Vec<String>>) {
        if left == s.len() {
            res.push(path.iter().map(|&v| String::from(v)).collect());
            return;
        }

        for right in left..s.len() {
            if Self::is_palindrome(left, right, s) {
                path.push(&s[left..=right]);
                Self::backtrace(right + 1, s, path, res);
                path.pop();
            }
        }
    }

    #[inline]
    fn is_palindrome(left: usize, right: usize, s: &str) -> bool {
        let forward = s[left..=right].as_bytes();
        return forward.iter().eq(forward.iter().rev());
    }
}

#[test]
fn leetcode131_t1() {
    let s = String::from("a");
    let r = Solution::partition(s);
    println!("{:?}", r);
}