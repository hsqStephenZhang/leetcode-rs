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
        let l = p.unwrap().borrow().val;
        let r = q.unwrap().borrow().val;
        let result = Self::inner(root, l, r);
        result.map(|v| v.0)
    }

    fn inner(
        root: Option<Rc<RefCell<TreeNode>>>,
        val1: i32,
        val2: i32,
    ) -> Option<(Rc<RefCell<TreeNode>>, i32)> {
        if let Some(root) = root {
            let left = Self::inner(root.borrow().left.clone(), val1, val2);
            let mut cnt = 0;
            if let Some((l, cnt1)) = left {
                if cnt1 == 2 {
                    return Some((l, 2));
                } else {
                    cnt += cnt1;
                }
            }

            if let Some((r, cnt2)) = Self::inner(root.borrow().right.clone(), val1, val2) {
                if cnt2 == 2 {
                    return Some((r, 2));
                } else {
                    cnt += cnt2;
                }
            }

            let v = root.borrow().val;
            return Some((
                root.clone(),
                if v == val1 || v == val2 { cnt + 1 } else { cnt },
            ));
        }
        None
    }
}

struct Solution2;

impl Solution2 {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let l = p.unwrap().borrow().val;
        let r = q.unwrap().borrow().val;
        let result = Self::inner(root, l, r);
        result
    }

    fn inner(
        root: Option<Rc<RefCell<TreeNode>>>,
        val1: i32,
        val2: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(r) = root {
            let v = r.borrow().val;
            if v == val1 || v == val2 {
                return Some(r);
            }
            let left = Self::inner(r.borrow().left.clone(), val1, val2);
            let right = Self::inner(r.borrow().right.clone(), val1, val2);
            return match (left, right) {
                (None, None) => None,
                (Some(t), None) | (None, Some(t)) => Some(t),
                (Some(t1), Some(t2)) => Some(r),
            };
        }
        None
    }
}

#[test]
fn leetcode() {
    let root = TreeNode::from_vec(OptionVec::new(vec![3, 5, 1, 6, 2, 0, 8, -1, -1, 7, 4]));
    let left = root.clone().map(|v| v.borrow().left.clone()).unwrap();
    let right = root.clone().map(|v| v.borrow().left.clone()).unwrap();
    let r = Solution::lowest_common_ancestor(root.clone(), left, right);
    // println!("{:?}", t);
    println!("{:?}", r);
}
