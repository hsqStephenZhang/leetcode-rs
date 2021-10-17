struct Solution;

#[allow(warnings)]
impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let sum: i32 = nums.iter().sum();
        let sum = sum as usize;
        if sum % 2 == 1 {
            return false;
        }
        let target = sum >> 1;
        let dp = vec![vec![0; nums.len()]; target];

        todo!()
    }
}

#[test]
fn leetcode416_t1() {
    todo!()
}
