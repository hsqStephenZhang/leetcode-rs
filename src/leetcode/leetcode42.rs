struct Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut sum = 0;

        let mut stack: Vec<(i32, usize)> = Vec::with_capacity(height.len());
        for (right_index, &right_val) in height.iter().enumerate() {
            while let Some(&v) = stack.last() {
                if v.0 < right_val {
                    let (mid_val, _) = stack.pop().unwrap();
                    match stack.last() {
                        Some(&(left_val, left_index)) => {
                            sum += (right_index - left_index - 1) as i32
                                * (left_val.min(right_val) - mid_val);
                        }
                        None => {}
                    }
                } else {
                    break;
                }
            }
            stack.push((right_val, right_index));
            // dbg!(sum);
        }

        sum as i32
    }
}

#[test]
fn leetcode42_t1() {
    let nums = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
    let r = Solution::trap(nums);
    println!("{:?}", r);

    let nums = vec![0, 1, 0, 2, 1, 1, 1, 3, 2, 1, 2, 1];
    let r = Solution::trap(nums);
    println!("{:?}", r);

    let nums = vec![0, 1, 2, 3, 4, 5];
    let r = Solution::trap(nums);
    println!("{:?}", r);

    let nums = vec![6, 5, 4, 3, 2, 1];
    let r = Solution::trap(nums);
    println!("{:?}", r);
}
