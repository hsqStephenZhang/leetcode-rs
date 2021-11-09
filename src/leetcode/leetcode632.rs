use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;

struct Solution;

#[derive(Clone, Debug, Default)]
struct Item {
    array_index: usize,
    val: i32,
}

impl Item {
    pub fn new(array_index: usize, val: i32) -> Self {
        Self {
            array_index,
            val,
        }
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.val.partial_cmp(&other.val)
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        self.val.cmp(&other.val)
    }
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.val.eq(&other.val)
    }
}

impl Eq for Item {}

struct SortMultiVec {
    vecs: Vec<Vec<i32>>,
}

impl SortMultiVec {
    pub fn new(vecs: Vec<Vec<i32>>) -> Self {
        debug_assert_eq!(None, vecs.iter().find(|v| v.len() == 0));
        Self {
            vecs
        }
    }

    pub fn sort(self) -> Vec<Item> {
        let length: usize = self.vecs.iter().map(|v| v.len()).sum();
        let mut r = Vec::with_capacity(length);
        let mut heap = BinaryHeap::new();
        for (array_index, v) in self.vecs.iter().enumerate() {
            heap.push((Reverse(Item::new(array_index, v[0])), 0));
        }
        while !heap.is_empty() {
            let (item, index) = heap.pop().unwrap();
            let array_index = item.0.array_index;
            r.push(item.0);
            let v = &self.vecs[array_index];

            if v.len() == index + 1 {
                continue;
            }
            heap.push((Reverse(Item::new(array_index, v[index + 1])), index + 1));
        }

        r
    }
}

impl Solution {
    pub fn smallest_range(nums: Vec<Vec<i32>>) -> Vec<i32> {
        struct R {
            left_val: i32,
            right_val: i32,
        }
        impl R {
            pub fn new(left_val: i32, right_val: i32) -> Self {
                Self { left_val, right_val }
            }

            pub fn update(&mut self, left_val: i32, right_val: i32) {
                if right_val - left_val < self.right_val - self.left_val {
                    *self = Self::new(left_val, right_val);
                }
            }
        }

        let length = nums.len();
        let mut array_cnt: Vec<i32> = vec![0; length];
        let mut total = 0;
        let s = SortMultiVec::new(nums);
        let sorted_items = s.sort();

        let mut res = R::new(i32::MIN / 2, i32::MAX / 2);

        // window
        let mut l = 0;
        for (r, &Item { array_index, val: current_val }) in sorted_items.iter().enumerate() {
            array_cnt[array_index] += 1;
            if array_cnt[array_index] == 1 {
                total += 1;
            }

            if total == length {
                res.update(sorted_items[l].val, current_val);
                // update
                while l < r {
                    let &Item { array_index: l_array_index, val: _ } = &sorted_items[l];
                    array_cnt[l_array_index] -= 1;
                    l += 1;
                    // 如果删除了 l 位置上的元素，仍然满足要求，就通过 l+1 位置上的元素来更新 res
                    if array_cnt[l_array_index] >= 1 {
                        res.update(sorted_items[l].val, current_val);
                    } else {
                        total -= 1;
                        break;
                    }
                }
            }
        }

        vec![res.left_val, res.right_val]
    }
}

#[test]
fn leetcode632() {
    // [0, 4, 5, 9, 10, 12, 15, 18, 20, 22, 24, 26, 30]
    let nums = vec![
        vec![4, 10, 15, 24, 26], vec![0, 9, 12, 20], vec![5, 18, 22, 30],
    ];
    // let nums=vec![vec![10,10],vec![11,11]];
    // let nums = vec![vec![1], vec![2], vec![3], vec![4], vec![5], vec![6]];
    // let s = SortMultiVec::new(vecs);
    // let r = s.sort();
    // let r = r.iter().map(|v| v.val).collect::<Vec<_>>();
    let r = Solution::smallest_range(nums);
    println!("{:?}", r);
}
