struct Solution;

impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();
        let mut res = Vec::with_capacity(8);
        let mut path = Vec::with_capacity(nums.len());
        let mut used = vec![false; nums.len()];
        Self::backtrace(&nums, &mut path, &mut used, &mut res);
        res
    }

    pub fn backtrace(
        nums: &Vec<i32>,
        path: &mut Vec<i32>,
        used: &mut Vec<bool>,
        res: &mut Vec<Vec<i32>>,
    ) {
        if path.len() == nums.len() {
            res.push(path.clone());
            return;
        }

        for i in 0..nums.len() {
            if i > 0 && nums[i] == nums[i - 1] && used[i - 1] == false {
                continue;
            }

            if used[i] == false {
                used[i] = true;
                path.push(nums[i]);
                Self::backtrace(nums, path, used, res);
                path.pop();
                used[i] = false;
            }
        }
    }
}

#[test]
fn leetcode47_t1() {
    let nums = vec![1, 1, 2, 2];
    let r = Solution::permute_unique(nums);
    println!("{:?}", r);
}
