use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = vec![];
        inner_traversal(root, &mut result);
        result
    }
}

fn inner_traversal(node: Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
    if let Some(mut n) = node {
        inner_traversal(n.borrow_mut().left.take(), res);
        res.push(n.borrow().val);
        inner_traversal(n.borrow_mut().right.take(), res);
    }
}

#[test]
fn leetcode() {
    let mut t = TreeNode::from_arr(&[Some(1), None, Some(2), Some(3)]);
    let r = Solution::inorder_traversal(t.clone());
    println!("{:?}", t);
    println!("{:?}", r);
}
