use std::cmp::Ordering;
use std::collections::BinaryHeap;

struct Solution;

struct Pair(i32, i32, i32);

impl Eq for Pair {}

impl PartialEq<Self> for Pair {
    fn eq(&self, other: &Self) -> bool {
        self.2.eq(&other.2)
    }
}

impl PartialOrd<Self> for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.2.cmp(&other.2))
    }
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> Ordering {
        self.2.cmp(&other.2)
    }
}

impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let mut heap: BinaryHeap<Pair> = BinaryHeap::new();
        let k = k as usize;
        for &v1 in nums1.iter().take(k) {
            for &v2 in nums2.iter().take(k) {
                let cur = Pair(v1, v2, v1 + v2);
                if heap.len() < k {
                    heap.push(cur);
                } else {
                    if let Ordering::Greater = heap.peek().unwrap().cmp(&cur) {
                        heap.pop();
                        heap.push(cur);
                    }
                }
            }
        }
        let mut r = heap.iter().collect::<Vec<_>>();
        r.sort();
        r.iter().map(|&p| vec![p.0, p.1]).collect()
    }
}

#[test]
fn leetcode373_t1() {
    let nums1 = vec![1, 7, 11];
    let nums2 = vec![2, 4, 6];
    let r = Solution::k_smallest_pairs(nums1, nums2, 9);
    println!("{:?}", r);
}
