struct Solution;

impl Solution {
    pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
        let mut nums =nums;
        Self::inner(&mut nums[..]);
        nums
    }

    fn inner(nums:&mut [i32]){
        let mid = nums.len() >> 1;
        if mid == 0 {
            return ;
        }
        Self::inner(&mut nums[..mid]);
        Self::inner(&mut nums[mid..]);

        let mut ret=nums.to_vec();
        Self::merge(&nums[0..mid], &nums[mid..], &mut ret[..]);
        nums.copy_from_slice(&ret[..]);
    }

    fn merge(nums1: &[i32], nums2: &[i32], res: &mut [i32]) {
        debug_assert!(nums1.len() + nums2.len() == res.len());
        let mut r = res.iter_mut();
        let mut i1 = nums1.iter().peekable();
        let mut i2 = nums2.iter().peekable();
        loop {
            let l1 = i1.peek();
            let l2 = i2.peek();
            match (l1, l2) {
                (None, None) => break,
                (Some(&&v), None) => {
                    *r.next().unwrap() = v;
                    i1.next();
                    break;
                }
                (None, Some(&&v)) => {
                    *r.next().unwrap() = v;
                    i2.next();
                    break;
                }
                (Some(&&v1), Some(&&v2)) => {
                    if v1 > v2 {
                        *r.next().unwrap() = *i2.next().unwrap();
                    } else {
                        *r.next().unwrap() = *i1.next().unwrap();
                    }
                }
            }
        }
        let remained = if i1.peek().is_none() { i2 } else { i1 };
        remained.for_each(|&v| {
            *r.next().unwrap() = v;
        });
    }
}

struct Solution2;

impl Solution2 {
    pub fn sort_array(mut nums: Vec<i32>) -> Vec<i32> {
       nums.sort_unstable();
       nums
    }
}

#[test]
fn leetcode912_t1() {
    let a = vec![5, 2, 3, 1, 4];
    assert_eq!(Solution::sort_array(a), vec![1, 2, 3, 4, 5]);
}
