struct Solution;

impl Solution {
    pub fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
        let mut res = 1;
        let mut left = 0;
        for (index, &val) in nums.iter().enumerate().skip(1) {
            if val <= nums[index - 1] {
                left = index;
            }
            res = res.max(index - left + 1);
        }

        res as i32
    }
}

#[test]
fn leetcode674_t1() {
    let nums = vec![1, 3, 5, 4, 7];
    let r = Solution::find_length_of_lcis(nums);
    println!("{}", r);
}
