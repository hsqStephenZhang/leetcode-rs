use std::cmp::Ordering;

struct Solution;

impl Solution {
    pub fn max_number(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut result: Vec<i32> = vec![];
        for i in 0..=k {
            if i > nums1.len() || k - i > nums2.len() {
                continue;
            }
            dbg!(i, k - i);
            let v1 = Self::max_number_of_arr(&nums1, i);
            let v2 = Self::max_number_of_arr(&nums2, k - i);
            dbg!(v1.clone(), v2.clone());
            let merged = Self::merge(&v1, &v2);
            result = match result.iter().cmp(merged.iter()) {
                Ordering::Equal => result,
                Ordering::Greater => result,
                Ordering::Less => merged,
            };
        }
        result
    }

    fn max_number_of_arr(nums: &Vec<i32>, remain: usize) -> Vec<i32> {
        if remain == 0 {
            return Vec::new();
        }
        let mut to_remove = (nums.len() - remain) as i32;
        let mut stack: Vec<i32> = Vec::new();
        nums.iter().for_each(|&x| {
            while to_remove > 0
                && match stack.last() {
                    None => false,
                    Some(&val) => val < x,
                }
            {
                stack.pop();
                to_remove -= 1;
            }
            stack.push(x);
        });
        stack.into_iter().take(remain).collect()
    }

    fn merge(nums1: &Vec<i32>, nums2: &Vec<i32>) -> Vec<i32> {
        let mut stack = Vec::with_capacity(nums1.len() + nums2.len());
        let mut i1 = nums1.iter().peekable();
        let mut i2 = nums2.iter().peekable();
        loop {
            let l1 = i1.peek();
            let l2 = i2.peek();
            match (l1, l2) {
                (None, None) => break,
                (Some(&&v), None) => {
                    stack.push(v);
                    i1.next();
                    break;
                }
                (None, Some(&&v)) => {
                    stack.push(v);
                    i2.next();
                    break;
                }
                (Some(&&v1), Some(&&v2)) => {
                    if v1 > v2 {
                        stack.push(*i1.next().unwrap());
                    } else if v1 < v2 {
                        stack.push(*i2.next().unwrap());
                    } else {
                        match i1.clone().cmp(i2.clone()) {
                            Ordering::Equal | Ordering::Less => stack.push(*i2.next().unwrap()),
                            Ordering::Greater => stack.push(*i1.next().unwrap()),
                        }
                    }
                }
            }
        }
        let res = stack
            .iter()
            .chain(if i1.peek().is_none() { i2 } else { i1 })
            .map(|&v| v)
            .collect();
        res
    }
}

#[test]
fn leetcode321_t1() {
    // let nums1 = vec![3, 4, 6, 5];
    // let nums2 = vec![9, 1, 2, 5, 8, 3];
    // let k = 5;
    // let r = Solution::max_number(nums1, nums2, k);
    // println!("{:?}", r);

    let nums1 = vec![8,1,8,8,6];
    let nums2 = vec![4];
    let k = 2;
    let r = Solution::max_number(nums1, nums2, k);
    println!("{:?}", r);
}
