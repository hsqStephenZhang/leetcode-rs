struct Solution;

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut dp: HashMap<i32, i32> = HashMap::new();
        let mut res = 0;
        for &num in nums.iter() {
            if dp.get(&num).is_some() {
                continue;
            }
            let left = *dp.get(&(num - 1)).unwrap_or(&0);
            let right = *dp.get(&(num + 1)).unwrap_or(&0);
            let cur = 1 + left + right;
            res = res.max(cur);
            dp.insert(num - left, cur);
            dp.insert(num, cur);
            dp.insert(num + right, cur);
        }
        res
    }

    pub fn longest_consecutive2(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut s = nums.into_iter().collect::<HashSet<_>>();
        for &val in s.iter() {
            if s.get(&(val - 1)).is_none() {
                let mut tmp = 1;
                let mut v = val + 1;
                while let Some(_) = s.get(&v) {
                    v += 1;
                    tmp += 1;
                }
                res = res.max(tmp);
            }
        }
        res
    }
}

#[test]
fn leetcode() {
    let nums = vec![1, 2, 0, 1, 3, 4, 5];
    let a = Solution::longest_consecutive2(nums);
    println!("{}", a);
}
