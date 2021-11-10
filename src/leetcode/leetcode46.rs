struct Solution;

impl Solution {
    /*
        take [1,2,3] as example,
        we should get the full permutation for the vec,
        the way to do it is traversing the dfs tree
    */

    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;

        let mut res = vec![];
        Self::backtrace(nums.len(), 0, &mut nums, &mut res);
        res
    }

    fn backtrace(length: usize, index: i32, nums: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
        if index as usize == length - 1 {
            res.push(nums.clone());
            return;
        }
        for j in (index as usize)..length {
            nums.swap(j, index as usize);
            Self::backtrace(length, index + 1, nums, res);
            nums.swap(j, index as usize);
        }
    }
}

struct Solution2;

impl Solution2 {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();

        let mut path = Vec::with_capacity(nums.len());

        let mut res = vec![];
        let mut used = vec![false; nums.len()];
        Self::backtrace(&mut path, &mut used, &nums, &mut res);
        res
    }

    fn backtrace(
        path: &mut Vec<i32>,
        used: &mut Vec<bool>,
        nums: &Vec<i32>,
        res: &mut Vec<Vec<i32>>,
    ) {
        if path.len() == nums.len() {
            res.push(path.clone());
            return;
        }

        // this solution have a worse time complexity
        for i in 0..nums.len() {
            if used[i] == false {
                used[i] = true;
                path.push(nums[i]);
                Self::backtrace(path, used, &nums, res);
                path.pop();
                used[i] = false;
            }
        }
    }
}

#[test]
fn leetcode46_t1() {
    let nums = vec![1, 2, 3];
    let r = Solution::permute(nums);
    println!("{:?}", r);
}

#[test]
fn leetcode46_t2() {
    let nums = vec![1, 2, 3];
    let r = Solution2::permute(nums);
    println!("{:?}", r);
}
