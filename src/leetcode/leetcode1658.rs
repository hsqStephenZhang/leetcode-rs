use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        let size = nums.len();
        let mut max_range = -1;
        let (sumed, sum) = Self::to_sum(nums);
        let target = sum - x;
        if target < 0 {
            return -1;
        }
        let mut map = HashMap::new();
        sumed
            .iter()
            .enumerate()
            .map(|(i, &val)| map.insert(val, i))
            .count();
        // dbg!(sumed.clone(), map.clone(), target);
        for (i, val) in sumed.into_iter().enumerate() {
            let key = val - target;
            let f = map.get(&key);
            match f {
                None => {}
                Some(&index) => {
                    let val = (index as i64 - i as i64).abs();
                    max_range = if max_range < val {
                        (index as i64 - i as i64).abs()
                    } else {
                        max_range
                    };
                }
            }
        }
        return if max_range == -1 {
            -1
        } else {
            (size - max_range as usize) as i32
        };
    }

    fn to_sum(nums: Vec<i32>) -> (Vec<i32>, i32) {
        let mut res = Vec::with_capacity(nums.len() + 1);
        let mut sum = 0;
        res.push(sum);
        for i in nums.into_iter() {
            sum += i;
            res.push(sum);
        }

        (res, sum)
    }
}

#[test]
fn leetcode1658_t1() {
    let nums = vec![1, 1, 4, 2, 3];
    let x = 5;
    debug_assert_eq!(2, Solution::min_operations(nums, x))
}

#[test]
fn leetcode1658_t2() {
    let nums = vec![5, 6, 7, 8, 9];
    let x = 4;
    debug_assert_eq!(-1, Solution::min_operations(nums, x))
}

#[test]
fn leetcode1658_t3() {
    let nums = vec![1, 2, 3, 4, 5];
    let x = 15;
    debug_assert_eq!(5, Solution::min_operations(nums, x))
}
