use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let positions = nums
            .iter()
            .enumerate()
            .filter(|&(_, &v)| v == 0)
            .map(|(i, _)| i)
            .collect::<Vec<_>>();

        let size = positions.len();

        if size <= k {
            return nums.len() as i32;
        }

        if k == 0 {
            let mut res = 0;
            for &val in positions.iter() {
                res = res.max(nums.iter().skip(val + 1).take_while(|&&x| x == 1).count());
            }
            return res as i32;
        }

        let start = positions[0];
        let end = positions[k - 1];
        let mut res = Self::range_sum(&nums, start, end);

        let mut selected_k: VecDeque<usize> = positions.iter().map(|&x| x).take(k).collect();

        for i in k..size {
            let end_val = positions[i];
            selected_k.push_back(end_val);
            selected_k.pop_front();

            let start = selected_k.front().unwrap();

            let tmp = Self::range_sum(&nums, *start, end_val);
            res = res.max(tmp);
        }

        res
    }

    pub fn range_sum(nums: &Vec<i32>, start: usize, end: usize) -> i32 {
        debug_assert!(end <= nums.len());
        let len1 = nums
            .iter()
            .rev()
            .skip(nums.len() - start)
            .take_while(|&&x| x == 1)
            .count();
        let len2 = nums.iter().skip(end + 1).take_while(|&&x| x == 1).count();
        // dbg!(len1, len2);
        return (end - start + len1 + len2 + 1) as i32;
    }
}

#[test]
fn leetcode1004_t1() {
    let nums = vec![0, 0, 1, 1, 1, 0, 0];
    let k = 0;
    let val = Solution::longest_ones(nums, k);
    println!("{}", val);
}
