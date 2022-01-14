struct Solution;

/*


(1): 1

(1, 2): 1*1 + 1*1

(1, 2, 3): 2*1 + 1*1 + 1*2

*/

impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        let mut dp = vec![1; n as usize +1 ];
        for i in 2..=n {
            let mut sum = 0;

            for j in 0..i {
                sum += dp[j as usize] * dp[(i - j - 1) as usize];
            }

            dp[i as usize] = sum;
        }
        dp[n as usize]
    }
}

#[test]
fn leetcode96_t1(){
    let n=10;
    dbg!(Solution::num_trees(n));
}
