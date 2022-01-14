use crate::utils::tree::{OptionVec, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, mut k: i32) -> i32 {
        let mut node = root;
        let mut prev;
        while let Some(n) = node.clone() {
            match n.borrow().left.clone() {
                Some(l) => {
                    prev = Some(l);
                    while let Some(p) = prev.clone() {
                        if let Some(r) = p.borrow().right.clone() {
                            if r != n {
                                prev = p.borrow().right.clone();
                                continue;
                            }
                        }
                        break;
                    }
                    // there must be one previous node
                    let p = prev.unwrap();
                    if p.borrow().right.is_none() {
                        p.borrow_mut().right = node;
                        node = n.borrow().left.clone();
                    } else {
                        k-=1;
                        if k==0{
                            return n.borrow().val
                        }
                        p.borrow_mut().right = None;
                        node = n.borrow().right.clone();
                    }
                }
                None => {
                    k-=1;
                    if k==0{
                        return n.borrow().val
                    }
                    node = n.borrow().right.clone();
                }
            }
        }
        unreachable!()
    }
}

#[test]
fn leetcode230_t1(){
    let root = TreeNode::from_vec(OptionVec::new(vec![3,1,4,-1,2]));
    let r=Solution::kth_smallest(root, 4);
    dbg!(r);
}