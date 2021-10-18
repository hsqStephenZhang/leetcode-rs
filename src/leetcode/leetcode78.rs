struct Solution;

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![vec![]];
        for val in nums.into_iter() {
            let a = res.clone();
            for mut subset in a.into_iter() {
                subset.push(val);
                res.push(subset);
            }
        }

        res
    }
}

#[test]
fn leetcode78_t1() {
    let nums = vec![1, 2, 3];
    let r = Solution::subsets(nums);
    println!("{:?}", r);
}
