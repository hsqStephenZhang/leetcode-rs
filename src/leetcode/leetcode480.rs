use crate::utils::multiset::*;
use std::cmp::Reverse;
use std::fmt::{Debug, Formatter};

struct Solution;

impl Solution {
    pub fn median_sliding_window(nums: Vec<i32>, k: i32) -> Vec<f64> {
        let k = k as usize;
        let mut window = Window::from_slice(&nums[0..k]);
        let mut res = Vec::with_capacity(nums.len() - k + 1);
        // dbg!(window.clone());
        res.push(window.mid());

        let left = nums.iter();
        let right = nums.iter().skip(k);
        for (&r, &l) in right.zip(left) {
            window.insert(r);
            window.delete(l);
            // dbg!(window.clone());
            res.push(window.mid());
        }
        res
    }
}

/// big.len() == small.len() || big.len() == small.len() + 1
#[derive(Clone, Default)]
struct Window {
    big: MultiSet<i32>,
    small: MultiSet<Reverse<i32>>,
}

impl Debug for Window {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "big:{:?} small:{:?}", self.big.clone(), self.small.clone())
    }
}

impl Window {
    pub fn new() -> Self {
        Self {
            big: Default::default(),
            small: Default::default(),
        }
    }

    pub fn from_slice(s: &[i32]) -> Self {
        let mut r = Self::new();
        for &v in s.iter() {
            r.insert(v);
        }
        r
    }

    pub fn insert(&mut self, val: i32) {
        let small_mid = self.small.peek();
        if let Some(&small_mid) = small_mid {
            if small_mid.0 > val {
                self.small.insert(Reverse(val));
            } else {
                self.big.insert(val);
            }
        } else {
            self.big.insert(val);
        }
        self.update();
    }

    pub fn delete(&mut self, key: i32) {
        let small_mid = self.small.peek();
        if let Some(&small_mid) = small_mid {
            if small_mid.0 >= key {
                self.small.remove(&Reverse(key));
            } else {
                self.big.remove(&key);
            }
        } else {
            self.big.remove(&key);
        }
        self.update();
    }

    fn update(&mut self) {
        if self.big.len() > self.small.len() + 1 {
            let big_mid = self.big.pop().unwrap();
            self.small.insert(Reverse(big_mid));
        } else if self.big.len() < self.small.len() {
            let small_mid = self.small.pop().unwrap();
            self.big.insert(small_mid.0);
        }
    }

    pub fn mid(&self) -> f64 {
        // dbg!(self.big.len(),self.small.len());
        if self.big.len() > self.small.len() {
            *self.big.peek().unwrap() as f64
        } else {
            (*self.big.peek().unwrap() as f64 + self.small.peek().unwrap().0 as f64) / 2f64
        }
    }
}

#[test]
fn leetcode480_t1() {
    // todo!()
    let arr = vec![1, 3, -1, -3, 5, 3, 6, 7];
    let k = 3;
    let r = Solution::median_sliding_window(arr, k);
    println!("{:?}", r);
}
