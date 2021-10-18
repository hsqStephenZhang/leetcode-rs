struct Solution;

impl Solution {
    /*
    [1,1,2]

    1,1,2
    1,2,1
    1,1,2 x
    1,2,1 x
    2,1,1
    2,1,1 x

    [1,1]
    1,1
    1,1   x
    */

    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();

        let mut path = Vec::with_capacity(nums.len());
        let mut used = vec![false; nums.len()];
        let mut res = vec![];
        Self::backtrace(&mut path, &mut nums, &mut used, &mut res);
        res
    }

    fn backtrace(
        path: &mut Vec<i32>,
        nums: &mut Vec<i32>,
        used: &mut Vec<bool>,
        res: &mut Vec<Vec<i32>>,
    ) {
        if path.len() == nums.len() {
            res.push(path.clone());
            return;
        }
        for i in 0..nums.len() {
            if used[i] == true || (i > 0 && nums[i] == nums[i - 1] && used[i - 1] == false) {
                continue;
            }
            used[i] = true;
            path.push(nums[i]);
            Self::backtrace(path, nums, used, res);
            path.pop();
            used[i] = false;
        }
    }
}

#[test]
fn leetcode47_t1() {
    let nums = vec![1, 1, 2, 2];
    let r = Solution::permute_unique(nums);
    println!("{:?}", r);
}
