use crate::utils::tree::{OptionVec, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::inner(root, i64::MIN, i64::MAX)
    }

    pub fn inner(root: Option<Rc<RefCell<TreeNode>>>, min_val: i64, max_val: i64) -> bool {
        if let Some(root) = root {
            let v = root.borrow().val as i64;
            return if v <= min_val || v >= max_val {
                false
            } else {
                Self::inner(root.borrow().left.clone(), min_val, v)
                    && Self::inner(root.borrow().right.clone(), v, max_val)
            };
        }
        true
    }
}

#[test]
fn leetcode() {
    let root = TreeNode::from_vec(OptionVec::new(vec![5, 1, 4, -1, -1, 3, 6]));
    // let root = TreeNode::from_vec(OptionVec::new(vec![2, 1, 3]));
    let r = Solution::is_valid_bst(root.clone());
    println!("{:?}", r);
}
