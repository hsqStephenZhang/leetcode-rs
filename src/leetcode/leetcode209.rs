struct Solution;

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = 0;
        let mut sum = 0;
        let mut min_range = i32::MAX;
        loop {
            while right < nums.len() {
                sum += nums[right];
                right += 1;
                if sum >= target {
                    break;
                }
            }
            if sum > target {
                min_range = min_range.min((right - left) as i32);
                while left < right {
                    sum -= nums[left];
                    left += 1;
                    if sum >= target {
                        min_range = min_range.min((right - left) as i32);
                    } else if sum < target {
                        break;
                    }
                }
            } else if sum == target {
                min_range = min_range.min((right - left) as i32);
            }

            if right == nums.len() {
                break;
            }
        }
        return if min_range == i32::MAX { 0 } else { min_range };
    }
}

#[test]
fn leetcode209_t1() {
    let a = vec![1, 2, 3, 4, 5];
    let r = Solution::min_sub_array_len(11, a);
    assert_eq!(r, 3);
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
