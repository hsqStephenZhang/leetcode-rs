use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

struct Solution;

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if root.is_none() {
            return vec![];
        }
        type Node = Rc<RefCell<TreeNode>>;
        let mut res = vec![];
        let mut queue: VecDeque<Node> = VecDeque::with_capacity(1);
        queue.push_back(root.unwrap());
        while queue.len() != 0 {
            let s = queue.len();
            let mut tmp: VecDeque<Node> = VecDeque::with_capacity(queue.len());
            let mut line = Vec::with_capacity(s);
            queue.into_iter().for_each(|v| {
                line.push(v.borrow_mut().val);
                if let Some(v) = v.borrow_mut().left.clone() {
                    tmp.push_back(v);
                }
                if let Some(v) = v.borrow_mut().right.clone() {
                    tmp.push_back(v);
                }
            });
            res.push(line);
            queue = tmp;
        }

        res
    }
}

#[test]
fn leetcode() {
    let inorder = vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)];
    let root = TreeNode::from_arr(inorder.as_slice());
    let r = Solution::level_order(root);
    println!("{:?}", r);
}
