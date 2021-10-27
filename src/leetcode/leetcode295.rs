use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct MedianFinder {
    // 大顶堆, min..mid
    big: BinaryHeap<i32>,
    // 小顶堆, max..mid
    small: BinaryHeap<Reverse<i32>>,
}

impl MedianFinder {
    fn new() -> Self {
        Self {
            big: BinaryHeap::new(),
            small: BinaryHeap::new(),
        }
    }

    fn add_num(&mut self, num: i32) {
        if self.small.is_empty() || num > self.small.peek().unwrap().0 {
            self.small.push(Reverse(num));
            if self.small.len() > self.big.len() + 1 {
                let r = self.small.pop();
                self.big.push(r.unwrap().0);
            }
        } else {
            self.big.push(num);
            if self.big.len() > self.small.len() {
                let r = self.big.pop();
                self.small.push(Reverse(r.unwrap()));
            }
        }
    }

    fn find_median(&self) -> f64 {
        // dbg!(self.small.clone(),self.big.clone());
        return if self.small.len() > self.big.len() {
            self.small.peek().unwrap().0 as f64
        } else {
            (self.small.peek().unwrap().0 as f64 + *self.big.peek().unwrap() as f64) / 2.0
        };
    }
}

#[test]
fn leetcode() {
    let mut s = MedianFinder::new();
    for i in 0..10 {
        s.add_num(i);
        println!("{}", s.find_median());
    }
}
