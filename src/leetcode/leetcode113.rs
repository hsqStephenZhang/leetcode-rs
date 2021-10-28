use crate::utils::tree::{OptionVec, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut path = vec![];
        return if let Some(r) = root {
            Self::inner(r, target_sum, 0, &mut path, &mut res);
            res
        } else {
            res
        };
    }

    fn inner(
        root: Rc<RefCell<TreeNode>>,
        target: i32,
        mut val: i32,
        path: &mut Vec<i32>,
        res: &mut Vec<Vec<i32>>,
    ) {
        let root_val = root.borrow().val;
        val += root_val;
        path.push(root_val);
        match (root.borrow().left.clone(), root.borrow().right.clone()) {
            (None, None) => {
                if val == target {
                    res.push(path.clone());
                }
            }
            (Some(v), None) | (None, Some(v)) => Self::inner(v, target, val, path, res),
            (Some(left), Some(right)) => {
                Self::inner(left, target, val, path, res);
                Self::inner(right, target, val, path, res);
            }
        }
        path.pop();
    }
}

#[test]
fn leetcode() {
    let preorder = vec![1, 2, 3];
    let t = TreeNode::from_vec(OptionVec::new(preorder));
    let r = Solution::path_sum(t.clone(), 3);
    println!("{:?}", r);
}
