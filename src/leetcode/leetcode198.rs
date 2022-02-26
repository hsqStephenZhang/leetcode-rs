struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut dp = vec![0; nums.len() + 1];
        dp[1] = nums[0];
        for i in 1..nums.len() {
            dp[i + 1] = dp[i].max(dp[i - 1] + nums[i]);
        }

        dp[nums.len()]
    }
}

#[test]
fn leetcode198_t1() {
    let nums = vec![2, 7, 9, 3, 1];
    let res = Solution::rob(nums);
    assert_eq!(res, 12);

    let nums = vec![1, 2, 3, 1];
    let res = Solution::rob(nums);
    assert_eq!(res, 4);
}
