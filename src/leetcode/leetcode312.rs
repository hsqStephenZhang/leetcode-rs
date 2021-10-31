use std::iter::Once;

struct Solution;

impl Solution {
    pub fn max_coins(nums: Vec<i32>) -> i32 {
        static LEFT_RIGHT: &[i32; 1] = &[1];
        // let mut c: Vec<i32> = LEFT_RIGHT.iter().chain(nums.iter()).chain(LEFT_RIGHT.iter()).collect();
        // let mut dp = vec![vec![nums.len() + 1]; nums.len() + 1];

        0
    }

    fn inner(nums: &[i32], dp: &mut Vec<Vec<i32>>, i: usize, j: usize) {}
}

#[test]
fn leetcode312_t1() {
    let nums = vec![3, 1, 5, 8];
    let iter = nums.iter().chain(nums.iter());
    iter.count();
}
