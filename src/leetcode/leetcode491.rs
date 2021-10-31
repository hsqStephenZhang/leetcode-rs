struct Solution;

/// 2^n?
/// prune the search tree
impl Solution {
    pub fn find_subsequences(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        res
    }
}

#[test]
fn leetcode491_t1() {
    let nums = vec![4, 6, 7, 7];
    let r = Solution::find_subsequences(nums);
    println!("{:?}", r);
}

#[test]
fn leetcode491_t2() {
    let nums = vec![2, 3, 1, 5]; // 2,3 | 2,5 | 3,5 | 2,3,5 | 1,5
    let r = Solution::find_subsequences(nums);
    println!("{:?}", r);
}
