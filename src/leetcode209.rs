struct Solution;

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut v = Vec::with_capacity(1 + nums.len());
        v.push(0);
        let mut sum = 0;
        for i in nums.iter() {
            sum += *i;
            v.push(sum);
        }
        println!("{:?}", v.clone());

        let mut min_range = usize::MAX;

        for (i, val) in v.iter().enumerate() {
            let t = target + *val;
            let index = v
                .iter()
                .enumerate()
                .skip(i)
                .find(|(_, &x)| x >= t)
                .map(|v| v.0);
            match index {
                Some(val) => {
                    let range = val - i;
                    dbg!(t, val, range);
                    min_range = if min_range < range { min_range } else { range };
                }
                None => {}
            }
        }

        if min_range == usize::MAX {
            0
        } else {
            min_range as i32
        }
    }
}

#[test]
fn leetcode209_t1() {
    let a = vec![2, 3, 1, 2, 4, 3];
    let r = Solution::min_sub_array_len(7, a);
    assert_eq!(r, 2);
}

#[test]
fn leetcode209_t2() {
    let a = vec![1; 10];
    let r = Solution::min_sub_array_len(11, a.clone());
    assert_eq!(r, 0);
    let r = Solution::min_sub_array_len(10, a.clone());
    assert_eq!(r, 10);

    let a = vec![1, 4, 4];
    let r = Solution::min_sub_array_len(4, a);
    assert_eq!(r, 1);
}
