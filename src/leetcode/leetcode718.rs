struct Solution;

impl Solution {
    pub fn find_length(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut res = 0;
        let rows = nums1.len();
        let cols = nums2.len();
        let mut dp = vec![vec![0; cols + 1]; rows + 1];

        for i in 1..rows + 1 {
            for j in 1..cols + 1 {
                if nums1[i - 1] == nums2[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1] + 1;
                    res = res.max(dp[i][j]);
                }
            }
        }

        res
    }
}

struct Solution2;

impl Solution2 {
    pub fn find_length(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        return if nums1.len() > nums2.len() {
            Self::slide_window(&nums1[..], &nums2[..])
        } else {
            Self::slide_window(&nums2[..], &nums1[..])
        };
    }

    fn slide_window(nums1: &[i32], nums2: &[i32]) -> i32 {
        debug_assert!(nums1.len() >= nums2.len());
        let size1 = nums1.len();
        let size2 = nums2.len();
        let delta = size1 - size2;
        let mut res = 0;

        for i in 1..size2 {
            res = res.max(Self::max_length_range(&nums1[..i], &nums2[size2 - i..]));
        }

        for i in 0..delta {
            res = res.max(Self::max_length_range(&nums1[i..size2 + i], &nums2[..]));
        }

        for i in 0..size2 - 1 {
            res = res.max(Self::max_length_range(
                &nums1[size1 - size2 + i..],
                &nums2[0..size2 - i],
            ));
        }

        res
    }

    fn max_length_range(nums1: &[i32], nums2: &[i32]) -> i32 {
        let mut res = 0;
        debug_assert_eq!(nums1.len(), nums2.len());
        let mut count = 0;
        for (val1, val2) in nums1.iter().zip(nums2.iter()) {
            if val1 == val2 {
                count += 1;
                res = res.max(count);
            } else {
                count = 0;
            }
        }
        res
    }
}

#[test]
fn leetcode718_t1() {
    let nums1 = vec![1, 2, 3, 2, 1, 4];
    let nums2 = vec![3, 2, 1, 4, 5];
    let r = Solution2::find_length(nums1, nums2);
    println!("{}", r);
}
