struct Solution;

impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let sum: i32 = nums.iter().sum();
        let sum = sum as usize;
        if sum % 2 == 1 {
            return false;
        }
        let target = sum >> 1;

        // dp[i] 表示: 容量为i的背包，能凑成的最大子集和为 dp[i]
        let mut dp = vec![0; 10001];

        for &val in nums.iter() {
            let val=val as usize;
            for j in (val..=target).rev() {
                dp[j] = dp[j].max(dp[j - val] + dp[val]);
            }
        }

        dp[target]==target
    }
}

#[test]
fn leetcode416_t1() {
    let mut nums = vec![1, 5, 5, 11];
    let r = Solution::can_partition(nums);
    println!("{}", r);
}
