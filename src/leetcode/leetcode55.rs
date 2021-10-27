struct Solution;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut max_jump = 0;
        let mut len = nums.len();
        if len <= 1 {
            return true;
        }
        for (index, val) in nums.into_iter().enumerate().take(len - 1) {
            if index > max_jump {
                return false;
            }
            max_jump = max_jump.max(index + val as usize);
            // dbg!(max_jump);
            if max_jump >= len - 1 {
                return true;
            }
        }
        false
    }
}

#[test]
fn leetcode55_t1() {
    let mut nums = vec![2, 3, 1, 1, 4, 1, 1, 1];
    let r = Solution::can_jump(nums);
    println!("{}", r);
}
