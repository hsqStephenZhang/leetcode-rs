struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut res = i32::MIN;
        for &val in nums.iter() {
            if sum > 0 {
                sum += val;
            } else {
                sum = val;
            }
            res = res.max(sum);
        }

        res
    }
}

#[test]
fn leetcode53_t1() {
    todo!()
}
