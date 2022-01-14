struct Solution;
/*

3, 30, 32, 5, 9

9 5 3 32 30

 */
impl Solution {
    pub fn largest_number(mut nums: Vec<i32>) -> String {
        nums.sort_by(|a, b| format!("{}{}", b, a).cmp(&format!("{}{}", a, b)));
        if nums[0] == 0 {
            return 0.to_string();
        }
        nums.into_iter().map(|i| i.to_string()).collect::<String>()
    }
}

#[test]
fn leetcode179_t1() {
    let nums = vec![3, 30, 32, 5, 9];
    let r = Solution::largest_number(nums);
    dbg!(r);
}
