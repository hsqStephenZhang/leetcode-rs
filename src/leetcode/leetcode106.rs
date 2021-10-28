use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

struct Solution;

impl Solution {
    // TODO review
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Data {
            postorder: postorder.as_slice(),
            inorder: inorder.as_slice(),
            num_to_index: inorder.iter().enumerate().map(|(i, &j)| (j, i)).collect(),
        }
        .solution(0, postorder.len() - 1, 0, inorder.len() - 1)
    }
}

struct Data<'a> {
    inorder: &'a [i32],
    postorder: &'a [i32],
    num_to_index: HashMap<i32, usize>,
}

impl<'a> Data<'a> {
    fn solution(
        &mut self,
        post_left: usize,
        post_right: usize,
        inorder_left: usize,
        inorder_right: usize,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if post_left > post_right {
            return None;
        }
        let root_val = self.postorder[post_right];
        let mut root = TreeNode::new(root_val);
        let inorder_root = self.num_to_index[&root_val];
        let right_subtree_size = inorder_right - inorder_root;

        if inorder_root > 0 && post_right >= right_subtree_size + 1 {
            root.left = self.solution(
                post_left,
                post_right - right_subtree_size - 1,
                inorder_left,
                inorder_root - 1,
            );
        }
        if post_right > 0 {
            root.right = self.solution(
                post_right - right_subtree_size,
                post_right - 1,
                inorder_root + 1,
                inorder_right,
            );
        }

        Some(Rc::new(RefCell::new(root)))
    }
}

#[test]
fn leetcode() {
    let inorder = vec![9, 3, 15, 20, 7];
    let postorder = vec![9, 15, 7, 20, 3];
    let r = Solution::build_tree(inorder, postorder);
    println!("{:?}", r);
}
