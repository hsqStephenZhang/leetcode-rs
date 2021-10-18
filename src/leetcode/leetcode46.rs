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

#[test]
fn leetcode46_t1() {
    let nums = vec![1, 2, 3];
    let r = Solution::permute(nums);
    println!("{:?}", r);
}
