use std::collections::HashMap;

struct Solution;

impl Solution {
    //! nums contains 0|1 only
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        let mut res = 0;

        let sums = Self::to_sum(nums);
        let mut map = HashMap::new();
        map.insert(0, 0);

        for (i, val) in sums.into_iter().enumerate() {
            let pre_index = map.get(&val);
            match pre_index {
                None => {
                    map.insert(val, i);
                }
                Some(index) => res = if res > i - *index { res } else { i - *index },
            }
        }
        res as i32
    }

    fn to_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut sum = 0;
        let mut v = Vec::with_capacity(nums.len() + 1);
        v.push(0);
        for val in nums.into_iter() {
            sum += if val == 0 { -1 } else { val };
            v.push(sum);
        }
        v
    }
}

#[test]
fn leetcode523_t1() {
    let v = vec![1, 0, 1, 1, 0, 0, 3, 4, 5, 6, 1, 0, 2, 0, 1];
    let res = Solution::find_max_length(v);
    println!("{}", res);
}
