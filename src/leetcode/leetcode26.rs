struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        let mut count = 1_usize;
        for i in 1..nums.len() {
            if nums[i] != nums[i - 1] {
                nums[count] = nums[i];
                count += 1;
            }
        }
        count as i32
    }
}

#[test]
fn leetcode() {
    let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4, 4, 4, 4];
    let r = Solution::remove_duplicates(&mut nums);
    println!("{:?}", r);
    println!("{:?}", nums.iter().take(r as usize).collect::<Vec<_>>())
}
