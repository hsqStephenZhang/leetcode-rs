struct Solution;

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let n = text1.len();
        let m = text2.len();
        let mut dp = vec![vec![0; m + 1]; n + 1];

        let text2_0 = text2.as_bytes()[0];
        for (i, &c) in text1.as_bytes().iter().enumerate() {
            if c == text2_0 {
                dp[i][0] = 1;
            } else if i != 0 {
                dp[i][0] = dp[i - 1][0];
            }
        }

        let text1_0 = text1.as_bytes()[0];
        for (j, &c) in text2.as_bytes().iter().enumerate() {
            if c == text1_0 {
                dp[0][j] = 1;
            } else if j != 0 {
                dp[0][j] = dp[0][j - 1];
            }
        }

        let t1 = text1.as_bytes();
        let t2 = text2.as_bytes();

        for i in 1..n {
            for j in 1..m {
                if t1[i] == t2[j] {
                    dp[i][j] = dp[i - 1][j - 1] + 1;
                } else {
                    dp[i][j] = dp[i - 1][j].max(dp[i][j - 1])
                }
            }
        }
        dp[n - 1][m - 1]
    }
}

#[test]
fn leetcode1143_t1() {
    let text1 = "abcde".into();
    let text2 = "ace".into();
    let r = Solution::longest_common_subsequence(text1, text2);
    println!("{}", r);
}
