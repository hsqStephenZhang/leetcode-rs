use std::{net::SocketAddr, vec};

struct Solution;

/*

1, 2, 3, 4

the dictionary order is a search tree

1 234
1 243
1 324
1 342
1 423
1 432

1, 2, 3, 4, 5

1 2345
1 2354
1 2435
1 2453
1 2534
1 2543
1 3245
1 3254

...

4 321

*/
impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let n = nums.len();
        let mut index = n as i32 - 2;
        while index >= 0 {
            if nums[index as usize] < nums[index as usize + 1] {
                break;
            }
            index -= 1;
        }

        if index >= 0 {
            let val = nums[index as usize];
            let mut idx = 0;
            let mut first_bigger = i32::MAX;
            for i in (index as usize + 1)..n {
                if nums[i] > val && nums[i] < first_bigger {
                    first_bigger = nums[i];
                    idx = i;
                }
            }
            nums[idx] = val;
            nums[index as usize] = first_bigger;

            let idx = idx as i32;
            if (idx as usize) != nums.len() - 1 {
                if val > nums[idx as usize - 1] {
                    let mut loc = 0;
                    for i in 0..(idx as usize) {
                        if nums[i] <= val {
                            loc = i;
                            break;
                        }
                    }
                    for i in ((loc + 1)..(idx as usize + 1)).rev() {
                        nums[i] = nums[i - 1];
                    }
                    nums[loc] = val;
                } else if val < nums[idx as usize + 1] {
                    let mut loc = 0;
                    for i in ((idx as usize)..nums.len()).rev() {
                        if nums[i] >= val {
                            loc = i;
                            break;
                        }
                    }
                    for i in (idx as usize)..loc {
                        nums[i] = nums[i + 1];
                    }
                    nums[loc] = val;
                }
            }
            Solution::quick_sort(&mut nums[(index as usize + 1)..n]);
        } else {
            Solution::quick_sort(nums);
        }
    }

    fn quick_sort(nums: &mut [i32]) {
        let n = nums.len();
        if n >= 2 && nums[n - 1] > nums[0] {
            return;
        }
        for i in 0..(n >> 1) {
            let tmp = nums[i];
            nums[i] = nums[n - i - 1];
            nums[n - i - 1] = tmp;
        }
    }
}

#[test]
fn leetcode31_t1() {
    let mut nums = vec![1, 2, 3, 4];
    for _ in 0..24 {
        let r = Solution::next_permutation(&mut nums);
        dbg!(&nums);
    }
    // let mut nums = vec![1, 2, 4, 3];
    // let r = Solution::next_permutation(&mut nums);
    // dbg!(&nums);
    // let mut nums = vec![1, 3, 2, 4];
    // let r = Solution::next_permutation(&mut nums);
    // dbg!(&nums);
    // let mut nums = vec![1, 3, 4, 2];
    // let r = Solution::next_permutation(&mut nums);
    // dbg!(&nums);
    // let mut nums = vec![2, 1, 2, 2, 2, 2, 2, 1];
    // Solution::next_permutation(&mut nums);
    // dbg!(&nums);
}
