struct Solution;

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut min_range = usize::MAX;
        let mut sum = 0;
        let mut left = 0;

        for right in 0..nums.len() {
            sum += nums[right];
            // for each position(left or right), the satisfied range is unique
            // if [left,right] satisfied the requirement, then we can safely move left forward
            // vice versa
            while sum >= target {
                sum -= nums[left];
                min_range = min_range.min(right - left + 1);
                left += 1;
            }
        }

        match min_range {
            usize::MAX => 0,
            _ => min_range as i32,
        }
    }
}

#[test]
fn leetcode209_t1() {
    let a = vec![2, 3, 1, 2, 4, 3];
    let r = Solution::min_sub_array_len(7, a);
    assert_eq!(r, 2);
}

#[test]
fn leetcode209_t2() {
    let a = vec![1; 10];
    let r = Solution::min_sub_array_len(11, a.clone());
    assert_eq!(r, 0);
    let r = Solution::min_sub_array_len(10, a.clone());
    assert_eq!(r, 10);

    let a = vec![1, 4, 4];
    let r = Solution::min_sub_array_len(4, a);
    assert_eq!(r, 1);
}
