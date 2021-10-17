struct Solution;

impl Solution {
    pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
        let size = nums.len();
        let mut left = i32::MAX;
        let mut right = i32::MIN;
        let mut minn = i32::MAX;
        let mut maxn = i32::MIN;
        nums.iter().enumerate().for_each(|(index, &x)| {
            if maxn > x {
                right = index as i32;
            } else {
                maxn = x;
            }

            if minn < nums[size - index - 1] {
                left = (size - index - 1) as i32;
            } else {
                minn = nums[size - index - 1];
            }
        });

        return if right == i32::MIN {
            0
        } else {
            right - left + 1
        };
    }
}

#[test]
fn leetcode581_t1() {
    let nums = vec![2, 6, 4, 8, 10, 9, 15];
    let r = Solution::find_unsorted_subarray(nums);
    assert_eq!(r, 5);

    let nums = vec![1, 2, 3, 4];
    let r = Solution::find_unsorted_subarray(nums);
    assert_eq!(r, 0);

    let nums = vec![1, 3, 2, 2, 2];
    let r = Solution::find_unsorted_subarray(nums);
    assert_eq!(r, 4);

    let nums = vec![1, 2, 3, 3, 3];
    let r = Solution::find_unsorted_subarray(nums);
    assert_eq!(r, 0);

    let nums = vec![3, 1];
    let r = Solution::find_unsorted_subarray(nums);
    assert_eq!(r, 2);

    let nums = vec![3, 2, 1];
    let r = Solution::find_unsorted_subarray(nums);
    assert_eq!(r, 3);

    let nums = vec![1, 3, 2, 3, 3];
    let r = Solution::find_unsorted_subarray(nums);
    assert_eq!(r, 2);
}
