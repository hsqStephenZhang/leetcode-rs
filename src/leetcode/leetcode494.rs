struct Solution;

#[allow(warnings)]
impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums;
        nums.sort();
        let dp = vec![vec![0; target as usize]; nums.len()];
        todo!()
    }
}

#[test]
fn leetcode494_t1() {
    todo!()
}
