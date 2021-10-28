use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

struct Solution;

impl Solution {
    // TODO review
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Data {
            preorder: preorder.as_slice(),
            inorder: inorder.as_slice(),
            d: inorder.iter().enumerate().map(|(i, &j)| (j, i)).collect(),
        }
        .solution(0, preorder.len() - 1, 0, inorder.len() - 1)
    }
}

struct Data<'a> {
    preorder: &'a [i32],
    inorder: &'a [i32],
    d: HashMap<i32, usize>,
}

impl<'a> Data<'a> {
    fn solution(
        &mut self,
        preorder_left: usize,
        preorder_right: usize,
        inorder_left: usize,
        inorder_right: usize,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder_left > preorder_right {
            return None;
        }
        let root_val = self.preorder[preorder_left];
        let mut root = TreeNode::new(root_val);
        let inorder_root = self.d[&root_val];
        let left_subtree_size = inorder_root - inorder_left;
        if inorder_root > 0 {
            root.left = self.solution(
                preorder_left + 1,
                preorder_left + left_subtree_size,
                inorder_left,
                inorder_root - 1,
            );
        }
        root.right = self.solution(
            preorder_left + 1 + left_subtree_size,
            preorder_right,
            inorder_root + 1,
            inorder_right,
        );
        Some(Rc::new(RefCell::new(root)))
    }
}

#[test]
fn leetcode() {
    let a = 0..1;
    let preorder = vec![3, 9, 20, 15, 7];
    let inorder = vec![9, 3, 15, 20, 7];
    let r = Solution::build_tree(preorder, inorder);
    println!("{:?}", r);
}
