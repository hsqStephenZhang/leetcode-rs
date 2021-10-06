struct Solution;

impl Solution {
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        if k <= 0 {
            return 0;
        }
        let k = k as i64;
        let mut res = 0;
        let mut left = 0;
        let mut right = 0;
        let size = nums.len();
        let mut product = 1 as i64;
        while right < size {
            product *= nums[right] as i64;
            right += 1;
            while left < right && product >= k {
                product /= nums[left] as i64;
                left += 1;
            }
            // res += dbg!(right) - dbg!(left);
            res += right - left;
        }
        res as i32
    }
}

#[test]
fn leetcode713_t1() {
    let nums = vec![10, 5, 2, 6];
    let k = 100;
    let r = Solution::num_subarray_product_less_than_k(nums, k);
    println!("{}", r);

    let nums = vec![10, 5, 2, 6];
    let k = 0;
    let r = Solution::num_subarray_product_less_than_k(nums, k);
    println!("{}", r);
}
