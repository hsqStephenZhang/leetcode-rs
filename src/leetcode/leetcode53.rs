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
struct Solution2;

// left,right,sum
impl Solution2 {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        todo!()
    }
}

#[test]
fn leetcode53_t1() {
    let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
    let r = Solution::max_sub_array(nums);
    dbg!(r);
}
