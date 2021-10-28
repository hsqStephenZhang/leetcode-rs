use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub trait NullNode {
    fn is_null(&self) -> bool;
}

impl NullNode for i32 {
    fn is_null(&self) -> bool {
        return self.eq(&-1);
    }
}

pub struct OptionVec<T> {
    inner: Vec<Option<T>>,
}

impl<T> OptionVec<T>
where
    T: NullNode,
{
    #[inline]
    pub fn new(v: Vec<T>) -> Self {
        let inner = v
            .into_iter()
            .map(|v| if v.is_null() { None } else { Some(v) })
            .collect();
        Self { inner }
    }
}

impl<T> From<Vec<T>> for OptionVec<T>
where
    T: NullNode,
{
    #[inline]
    fn from(v: Vec<T>) -> Self {
        Self::new(v)
    }
}

pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    pub fn with_child(
        val: i32,
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) -> Self {
        Self { val, left, right }
    }

    #[inline]
    pub fn set_left(&mut self, left: Option<Rc<RefCell<TreeNode>>>) {
        if let Some(v) = left {
            self.left.replace(v);
        } else {
            self.left.take();
        }
    }

    #[inline]
    pub fn set_right(&mut self, right: Option<Rc<RefCell<TreeNode>>>) {
        if let Some(v) = right {
            self.right.replace(v);
        } else {
            self.right.take();
        }
    }

    ///
    /// ```rust
    /// let v = &[Some(1), None, Some(2), Some(3)];
    /// assert_eq!(1, 1);
    /// ```
    pub fn from_arr(v: &[Option<i32>]) -> Option<Rc<RefCell<Self>>> {
        return if v.len() == 0 {
            None
        } else {
            let root = Rc::new(RefCell::new(TreeNode::new(v[0].unwrap())));
            let mut queue = VecDeque::from([root.clone()]);
            let mut i = v.iter().skip(1);
            while let Some(&v) = i.next() {
                let parent = queue.pop_front().unwrap();
                if let Some(a) = v {
                    let left = Rc::new(RefCell::new(TreeNode::new(a)));
                    parent.borrow_mut().left.replace(left.clone());
                    queue.push_back(left.clone());
                }
                if let Some(&Some(b)) = i.next() {
                    let right = Rc::new(RefCell::new(TreeNode::new(b)));
                    parent.borrow_mut().right.replace(right.clone());
                    // parent.set_left(Some(right));
                    queue.push_back(right.clone());
                }
            }
            Some(root)
        };
    }

    #[inline]
    pub fn from_vec(v: OptionVec<i32>) -> Option<Rc<RefCell<Self>>> {
        Self::from_arr(v.inner.as_slice())
    }

    pub fn to_graphviz(&self, title: &str) -> String {
        let mut a = format!("digraph {} ", title);
        a.push_str("{");

        // bfs traverse

        a.push_str("}");
        a
    }
}