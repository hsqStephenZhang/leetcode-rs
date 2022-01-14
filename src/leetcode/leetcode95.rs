use crate::utils::tree::{OptionVec, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;
impl Solution {
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        TreeNode::generate_trees(n)
    }
}

type TreeRoot = Option<Rc<RefCell<TreeNode>>>;

impl TreeNode {
    pub fn generate_trees(n: i32) -> Vec<TreeRoot> {
        if n > 0 {
            Self::generate_trees_help(1, n)
        } else {
            Vec::new()
        }
    }

    fn generate_trees_help(start: i32, end: i32) -> Vec<TreeRoot> {
        let mut trees = Vec::new();
        if start > end {
            trees.push(None);
            return trees;
        }

        for k in start..=end {
            let left_trees = Self::generate_trees_help(start, k - 1);
            let right_trees = Self::generate_trees_help(k + 1, end);
            for i in left_trees.iter() {
                for j in right_trees.iter() {
                    let mut node = TreeNode::new(k);
                    node.left = i.clone();
                    node.right = j.clone();
                    trees.push(Some(Rc::new(RefCell::new(node))));
                }
            }
        }

        trees
    }
}

#[test]
fn leetcode95_t1() {
    let n = 3;
    dbg!(Solution::generate_trees(n));
}
