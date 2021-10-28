use crate::utils::tree::{OptionVec, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        Self::inner(root.unwrap(), target_sum, 0)
    }

    fn inner(root: Rc<RefCell<TreeNode>>, target: i32, mut val: i32) -> bool {
        let root_val = root.borrow().val;
        val += root_val;
        match (root.borrow().left.clone(), root.borrow().right.clone()) {
            (None, None) => {
                return val == target;
            }
            (Some(v), None) | (None, Some(v)) => Self::inner(v, target, val),
            (Some(left), Some(right)) => {
                Self::inner(left, target, val) || Self::inner(right, target, val)
            }
        }
    }
}

#[test]
fn leetcode() {
    let preorder = vec![1, 2, 3];
    let t = TreeNode::from_vec(OptionVec::new(preorder));
    let r = Solution::has_path_sum(t.clone(), 4);
    println!("{:?}", r);
}
