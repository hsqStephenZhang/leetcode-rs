struct Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut nums = nums;
        let length = nums.len();
        nums.sort();

        let mut start = 0;
        loop {
            if start >= length {
                break;
            }

            let mut left = start + 1;
            let mut right = length - 1;
            let mut target = -nums[start];

            loop {
                if left >= right {
                    break;
                }
                let v = nums[left] + nums[right];
                if v > target {
                    right -= 1;
                } else if v < target {
                    left += 1;
                } else {
                    let l = nums[left];
                    let r = nums[right];
                    res.push(vec![-target, l, r]);
                    while left < right && left < length && nums[left] == l {
                        left += 1;
                    }

                    while right > left && nums[right] == r {
                        right -= 1;
                    }
                }
            }
            while start < length && nums[start] == -target {
                start += 1;
            }
        }

        res
    }
}

#[test]
fn leetcode15_t1() {
    let nums = vec![-1, -1, -1, 0, 0, 0, 1, 1, 2];
    // -1 -1 2
    // -1 0 1
    // 0 0 0
    let r = Solution::three_sum(nums);
    println!("{:?}", r);
}
