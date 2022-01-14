use std::vec;

struct Solution;

/*

pos - neg = target

pos + neg = sum

===>

neg = (sum-target)/ 2

(sum-target) must be a even number

*/

impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums;
        let sum: i32 = nums.iter().sum();
        let diff = sum - target;
        if diff < 0 || diff/2 > sum || diff % 2 == 1 {
            return 0;
        }
        let neg_sum = diff / 2;
        // dp[i][j]: number of ways to select a subset, which sum as j in first i elements
        // dp[i][j]=dp[i-1][j]
        // dp[i][j]=dp[i-1][j] + dp[i-1][j-nums[i]]  if j>=nums[i]
        let mut dp = vec![vec![0; sum as usize + 1]; 1 + nums.len()];
        dp[0][0] = 1;
        for i in 1..(nums.len() + 1) {
            let val = nums[i - 1];
            for j in 0..=neg_sum {
                dp[i as usize][j as usize] = dp[i as usize - 1][j as usize];
                if j >= val {
                    dp[i as usize][j as usize] += dp[i as usize - 1][(j - val) as usize];
                }
            }
        }
        return dp[nums.len()][neg_sum as usize];
    }
}

#[test]
fn leetcode494_t1() {
    let nums = vec![100];
    let r = Solution::find_target_sum_ways(nums, -100);
    dbg!(r);
}
