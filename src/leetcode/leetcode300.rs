struct Solution;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut res = 0;

        let mut stack = Vec::<i32>::new();

        // greed + binary_search
        nums.iter().for_each(|&val| match stack.last() {
            None => {
                stack.push(val);
                res = 1;
            }
            Some(&v) => {
                if v < val {
                    stack.push(val);
                    res += 1;
                } else {
                    let index = stack.as_slice().binary_search(&val);
                    if let Err(i) = index {
                        stack[i] = val;
                    }
                }
            }
        });

        res as i32
    }
}

#[test]
fn leetcode300_t1() {
    let nums = vec![3, 2, 1, 8, 9, 3, 2, 4, 5];
    let r = Solution::length_of_lis(nums);
    println!("{}", r);
}

#[test]
fn ttt() {
    let a = vec![0, 3, 5, 7, 9];
    let s = &a[..];
    let v = s.binary_search(&2);
    println!("{:?}", v);
}
