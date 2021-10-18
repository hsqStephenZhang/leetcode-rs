struct Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut index = 0_usize;
        for i in 0..nums.len() {
            if nums[i] != 0 {
                nums[index] = nums[i];
                index += 1;
            }
        }
        nums.iter_mut().skip(index).for_each(|v| *v = 0);
    }
}

struct Solution2;

impl Solution2 {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut index = 0_usize;
        for i in 0..nums.len() {
            if nums[i] != 0 {
                nums[index] = nums[i];
                if index != i {
                    nums[i] = 0;
                }
                index += 1;
            }
        }
    }
}

#[test]
fn leetcode283_t1() {
    let mut nums = vec![0, 1, 0, 2, 12];
    let r = Solution::move_zeroes(&mut nums);
    println!("{:?}", nums);
}
