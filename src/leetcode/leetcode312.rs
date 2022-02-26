use std::iter::Once;

struct Solution;

impl Solution {
    // f(left, right) = max(f(left, mid) + f(mid, right) + nums[i] * nums[mid] * nums[j])
    pub fn max_coins(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut rec = vec![vec![0; n + 2]; n + 2];
        let val = (1..=1)
            .chain(nums.drain(..))
            .chain(1..=1)
            .collect::<Vec<i32>>();

        for i in (0..n).rev() {
            for j in (i + 2)..(n + 2) {
                for k in (i + 1)..j {
                    rec[i][j] = rec[i][j].max(val[i] * val[k] * val[j] + rec[i][k] + rec[k][j])
                }
            }
        }
        rec[0][n + 1]
    }
}

#[test]
fn leetcode312_t1() {
    let nums = vec![3, 1, 5, 8];
    assert_eq!(Solution::max_coins(nums), 167);
}
