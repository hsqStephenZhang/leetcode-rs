struct Solution;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        if nums.len() <= 1 {
            return 1;
        }
        let mut count = 0;
        let len = nums.len();
        let mut max_jump = 0;
        let mut end = 0;
        for (index, &val) in nums.iter().take(len - 1).enumerate() {
            max_jump = max_jump.max(index + val as usize);
            if index == end {
                end = max_jump;
                count += 1;
            }
        }

        count
    }
}

#[test]
fn leetcode45_t1() {
    let nums = vec![2, 3, 1, 1, 4];
    let r = Solution::jump(nums);
    println!("{}", r);
}
