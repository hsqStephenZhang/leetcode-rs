struct Solution;

impl Solution {
    pub fn num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32 {
        let size = nums.len();

        let mut left1 = 0;
        let mut left2 = 0;
        let mut right = 0;
        let mut sum1 = 0;
        let mut sum2 = 0;
        let mut res = 0;

        while right < size {
            sum1 += nums[right];
            sum2 += nums[right];

            while left1 <= right && sum1 > goal {
                sum1 -= nums[left1];
                left1 += 1;
            }

            // notice that, here the condition is sum2>=goal,
            // so we can cover the range where nums[left2]=0
            while left2 <= right && sum2 >= goal {
                sum2 -= nums[left2];
                left2 += 1;
            }

            res += left2 - left1;
            right += 1;
        }

        res as i32
    }
}

#[test]
fn leetcode930_t1() {
    let a = vec![1, 0, 1, 0, 1];
    let r = Solution::num_subarrays_with_sum(a, 2);
    println!("{}", r);
}
