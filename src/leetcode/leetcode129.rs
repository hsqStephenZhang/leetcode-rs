use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

impl Solution {
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut sum = 0;
        Self::inner(root.unwrap(), &mut sum, 0);
        sum
    }

    fn inner(root: Rc<RefCell<TreeNode>>, sum: &mut i32, mut val: i32) {
        let root_val = root.borrow().val;
        val = val * 10 + root_val;
        match (root.borrow().left.clone(), root.borrow().right.clone()) {
            (None, None) => {
                *sum += val;
            }
            (Some(v), None) | (None, Some(v)) => {
                Self::inner(v, sum, val);
            }
            (Some(left), Some(right)) => {
                Self::inner(left, sum, val);
                Self::inner(right, sum, val);
            }
        }
    }
}

#[test]
fn leetcode() {
    // let preorder = vec![Some(1), Some(2), Some(3)];
    let preorder = vec![Some(4), Some(9), Some(0), Some(5), Some(1)];
    let t = TreeNode::from_arr(preorder.as_slice());
    let r = Solution::sum_numbers(t.clone());
    println!("{:?}", t);
    println!("{:?}", r);
}
