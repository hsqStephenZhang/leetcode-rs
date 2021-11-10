struct Solution;

impl Solution {
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();

        let mut res = Vec::with_capacity(10);

        let mut used = vec![false; nums.len()];
        let mut path = Vec::with_capacity(nums.len());
        Self::backtrace(0, &nums, &mut path, &mut used, &mut res);
        res
    }

    pub fn backtrace(
        index: usize,
        nums: &Vec<i32>,
        path: &mut Vec<i32>,
        used: &mut Vec<bool>,
        res: &mut Vec<Vec<i32>>,
    ) {
        if index == nums.len() {
            res.push(path.clone());
            return;
        }

        Self::backtrace(index + 1, nums, path, used, res);
        if index > 0 && nums[index - 1] == nums[index] && used[index - 1] == false {
            return;
        }

        if used[index] == false {
            used[index] = true;
            path.push(nums[index]);
            Self::backtrace(index + 1, nums, path, used, res);
            path.pop();
            used[index] = false;
        }
    }
}

#[test]
fn leetcode90_t1() {
    let nums = vec![1, 2, 2];
    let r = Solution::subsets_with_dup(nums);
    println!("{:?}", r);
}
