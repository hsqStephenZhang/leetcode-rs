use std::vec;

struct Solution;

impl Solution {
    pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort();
        return nums[nums.len() - k as usize];
    }
}

#[test]
fn offer76_t1() {
    let nums = vec![1, 2, 3, 4, 5, 6];
    let r = Solution::find_kth_largest(nums, 2);
    dbg!(r);
}
