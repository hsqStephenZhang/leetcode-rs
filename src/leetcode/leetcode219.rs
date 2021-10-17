use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut m = HashMap::<i32, usize>::new();
        for (i, &val) in nums.iter().enumerate() {
            let r = m.get(&val);
            match r {
                None => {
                    m.insert(val, i);
                }
                Some(&idx) => {
                    if i - idx <= k as usize {
                        return true;
                    }
                    m.insert(val, i);
                }
            }
        }
        false
    }
}

#[test]
fn leetcode219_t1() {
    let nums = vec![1, 2, 3, 1];
    let k = 3;
    let r = Solution::contains_nearby_duplicate(nums, k);
    println!("{}", r);
}
