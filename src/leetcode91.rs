struct Solution;

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let bytes = s.as_str();
        let size = bytes.len();
        let mut dp = vec![0; size];
        let mut i = (size - 1) as i32;
        while i >= 0 {
            let index = i as usize;
            let r1 = Self::parse(&bytes[index..=index]);
            let r2 = if index == (size - 1) {
                None
            } else {
                Self::parse(&bytes[index..=(index + 1)])
            };

            match r1 {
                None => {}
                Some(_) => {
                    if index == (size - 1) {
                        dp[index] += 1;
                    } else {
                        dp[index] += dp[(index + 1)];
                    }
                }
            }
            match r2 {
                None => {}
                Some(_) => {
                    if index < (size - 2) {
                        dp[index] += dp[index + 2]
                    } else {
                        dp[index] += 1;
                    }
                }
            }

            dbg!(dp.clone());

            i -= 1;
        }
        return dp[0];
    }

    fn parse(s: &str) -> Option<i32> {
        if s.starts_with("0") {
            return None;
        }
        let r = s.parse::<i32>().unwrap();
        if r >= 1 && r <= 26 {
            Some(r)
        } else {
            None
        }
    }
}

#[test]
fn leetcode91_t1() {
    let s = "1234".into();
    let r = Solution::num_decodings(s);
    assert_eq!(3, r);
}
