struct Solution;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let m = p.len();
        let n = s.len();

        let mut dp: Vec<Vec<bool>> = vec![vec![false; n + 1]; m + 1];
        dp[0][0] = true;

        for (i, &a) in p.as_bytes().iter().enumerate() {
            if i >= 1 && a == b'*' {
                dp[i + 1][0] = dp[i - 1][0];
            }
        }

        for (i, &a) in p.as_bytes().iter().enumerate() {
            for (j, &b) in s.as_bytes().iter().enumerate() {
                if a == b'*' {
                    if Self::match_char(p.as_bytes()[i - 1], b) {
                        dp[i + 1][j + 1] = dp[i - 1][j + 1] | dp[i + 1][j] | dp[i - 1][j];
                    } else {
                        dp[i + 1][j + 1] = dp[i - 1][j + 1];
                    }
                } else {
                    dp[i + 1][j + 1] = if Self::match_char(a, b) && dp[i][j] {
                        true
                    } else {
                        false
                    }
                }
            }
        }

        dp[m][n]
    }

    #[inline(always)]
    fn match_char(a: u8, b: u8) -> bool {
        return a == b || a == b'.';
    }
}

#[test]
fn leetcode10_t1() {
    let s = String::from("aaab");
    let p = String::from("a*.");
    let r = Solution::is_match(s, p);
}
