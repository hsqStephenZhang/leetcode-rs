use crate::utils::tree::{OptionVec, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;
impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return None;
        }
        let mut path1 = Vec::with_capacity(8);
        let mut path2 = Vec::with_capacity(8);
        let target1 = p.unwrap().borrow().val;
        let target2 = q.unwrap().borrow().val;

        let mut n1 = root.clone();
        let mut n2 = root.clone();

        loop {
            let parent = n1.unwrap();
            let cur = parent.borrow().val;
            path1.push(parent.clone());
            if cur == target1 {
                break;
            } else if cur > target1 {
                n1 = parent.borrow().left.clone();
            } else {
                n1 = parent.borrow().right.clone();
            }
        }

        loop {
            let parent = n2.unwrap();
            let cur = parent.borrow().val;
            path2.push(parent.clone());
            if cur == target2 {
                break;
            } else if cur > target2 {
                n2 = parent.borrow().left.clone();
            } else {
                n2 = parent.borrow().right.clone();
            }
        }

        let max_index = path1.len().min(path2.len());
        let mut res = None;
        for index in 0..max_index {
            if path1[index] == path2[index] {
                res = Some(path1[index].clone());
            }
        }
        return res;
    }
}

#[test]
fn leetcode245_t1() {
    let root = TreeNode::from_vec(OptionVec::new(vec![6, 2, 8, 0, 4, 7, 9, -1, -1, 3, 5]));
    let p = root.clone().unwrap().borrow().left.clone();
    let q = p.clone().unwrap().borrow().right.clone();
    let r = Solution::lowest_common_ancestor(root, p, q);
    dbg!(r);
}
