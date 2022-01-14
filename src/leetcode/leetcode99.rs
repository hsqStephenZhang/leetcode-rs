use crate::utils::tree::{OptionVec, TreeNode};
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

struct Solution;

/*

right: 1, 2, 3
wrong: 3, 2, 1

right: 1, 2, 3, 4
wrong: 1, 3, 2, 4

 */
impl Solution {
    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        // let mut first = None;
        // let mut second = None;
        // Solution::dfs(root, &mut first, &mut second);
        let mut root = Some(root.as_ref().unwrap().clone());
        let mut queue = VecDeque::new();
        let mut x = None;
        let mut y = None;
        let mut pred: Option<Rc<RefCell<TreeNode>>> = None;
        while !queue.is_empty() || root.is_some() {
            while let Some(r) = root {
                queue.push_back(r.clone());
                root = r.borrow().left.clone();
            }

            let new_root = queue.pop_back().unwrap();
            if let Some(p) = pred {
                if new_root.borrow().val < p.borrow().val {
                    y = Some(new_root.clone());
                    if x.is_none() {
                        x = Some(p.clone());
                    } else {
                        break;
                    }
                }
            }
            pred = Some(new_root.clone());
            root = new_root.borrow().right.clone();
        }

        let x = x.unwrap();
        let y = y.unwrap();

        let tmp = x.borrow().val;
        x.borrow_mut().val = y.borrow().val;
        y.borrow_mut().val = tmp;
    }
}

#[test]
fn leetcode99_t1() {
    // let mut root = TreeNode::from_vec(OptionVec::new(vec![1, 3, -1, -1, 2]));
    let mut root = TreeNode::from_vec(OptionVec::new(vec![3, 1, 4, -1, -1, 2]));
    root.unwrap().borrow().morris_inverse_traverse();
    // dbg!(root);
    // Solution::recover_tree(&mut root);
    // dbg!(root);
}
