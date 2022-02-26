struct Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut total = (m + n) as usize;
        debug_assert!(total == nums1.capacity());

        let mut c1 = m;
        let mut c2 = n;
        while c1 > 0 && c2 > 0 {
            if nums1[c1 as usize - 1] > nums2[c2 as usize - 1] {
                nums1[total - 1] = nums1[c1 as usize - 1];
                c1 -= 1;
            } else {
                nums1[total - 1] = nums2[c2 as usize - 1];
                c2 -= 1;
            }
            total -= 1;
        }

        if c2> 0 {
            nums1[..c2 as usize].copy_from_slice(&nums2[..c2 as usize]);
        } else if c1 > 0 {
            for i in c1 as usize..total {
                nums1[i] = nums1[i - 1];
            }
        }
    }
}

#[test]
fn leetcode88_t1() {
    let mut nums1 = vec![0];
    let m = 0;
    let mut nums2 = vec![2];
    let n = 1;
    let r = Solution::merge(&mut nums1, m, &mut nums2, n);
    assert_eq!(nums1, vec![2]);
}
