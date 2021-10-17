struct Solution;

use std::collections::BTreeSet;

impl Solution {
    pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, k: i32, t: i32) -> bool {
        let k = k as usize;
        let t = t as i64;
        let mut tree = BTreeSet::new();

        for i in 0..nums.len() {
            let target = nums[i] as i64;
            // find the left and right element of target in the BTreeSet
            let (l, r) = neighbors(&mut tree, target);
            if let Some(val) = l {
                if target - val <= t {
                    return true;
                }
            }
            if let Some(val) = r {
                if val - target <= t {
                    return true;
                }
            }
            tree.insert(target);

            if i >= k {
                tree.remove(&(nums[i - k] as i64));
            }
        }
        false
    }
}

fn neighbors<T: Ord + Copy>(nums: &mut BTreeSet<T>, val: T) -> (Option<T>, Option<T>) {
    let mut left = nums.range(..&val);
    let mut right = nums.range(&val..);
    // dbg!(nums.clone(), left.clone(), right.clone());
    (left.next_back().map(|&x| x), right.next().map(|&x| x))
}

#[test]
fn leetcode220_t1() {
    let nums = vec![1, 2, 3, 1];
    let k = 3;
    let t = 0;
    let r = Solution::contains_nearby_almost_duplicate(nums, k, t);
    println!("{}", r);
}

#[test]
fn ttt() {
    let mut a = BTreeSet::new();
    for i in 0..10 {
        a.insert(i);
    }
    let l = a.range(..=&2);
    let r = a.range(&5..);
    dbg!(l, r);
}
