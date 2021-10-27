struct Solution;

use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap};

struct Pair(i32, usize);

impl Pair {
    pub fn new(v: i32, c: usize) -> Self {
        Self(v, c)
    }
}

impl Eq for Pair {}

impl PartialEq<Self> for Pair {
    fn eq(&self, other: &Self) -> bool {
        self.1.eq(&other.1)
    }
}

impl PartialOrd<Self> for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.1.cmp(&other.1))
    }
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> Ordering {
        self.1.cmp(&other.1)
    }
}

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut count: HashMap<i32, usize> = HashMap::new();
        nums.iter().for_each(|v| match count.get_mut(v) {
            Some(v) => *v += 1,
            None => {
                count.insert(*v, 1);
            }
        });
        // dbg!(count.clone());
        let mut heap = BinaryHeap::new();
        for (&v, &c) in count.iter() {
            let cur = Pair::new(v, c);
            if heap.len() < k {
                heap.push(Reverse(cur));
            } else {
                match heap.peek().unwrap().0.cmp(&cur) {
                    Ordering::Less => {
                        heap.pop();
                        heap.push(Reverse(cur));
                    }
                    _ => {}
                }
            }
        }
        // debug_assert!(heap.len() == k);
        heap.iter().map(|v| v.0 .0).collect()
    }
}

#[test]
fn leetcode347_t1() {
    let nums = vec![1, 1, 1, 2, 2, 3];
    let r = Solution::top_k_frequent(nums, 2);
    println!("{:?}", r);
}
