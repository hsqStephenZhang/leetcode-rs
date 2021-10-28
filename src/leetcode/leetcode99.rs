use crate::utils::tree::{OptionVec, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

impl Solution {
    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        todo!()
    }
}

#[test]
fn leetcode() {
    let mut root = TreeNode::from_vec(OptionVec::new(vec![5, 1, 4, -1, -1, 3, 6]));
    // let root = TreeNode::from_vec(OptionVec::new(vec![2, 1, 3]));
    let r = Solution::recover_tree(&mut root);
    println!("{:?}", r);
}
