use std::collections::BTreeMap;
use std::fmt::Formatter;
#[allow(unused_imports)]
#[allow(dead_code)]
use std::cmp::Reverse;

#[derive(Clone)]
pub struct MultiSet<K> {
    inner: BTreeMap<K, i32>,
    len: usize,
}

impl<K> Default for MultiSet<K>
where
    K: Ord,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<K> std::fmt::Debug for MultiSet<K>
where
    K: std::fmt::Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "len:{},content:{:?}",
            self.len,
            self.inner.iter().map(|v| v.0).collect::<Vec<&K>>()
        )
    }
}

// #[allow(mutable_borrow_reservation_conflict)]
impl<K> MultiSet<K>
where
    K: Ord,
{
    pub fn new() -> Self {
        Self {
            inner: BTreeMap::new(),
            len: 0,
        }
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn from_vec(slice: Vec<K>) -> Self {
        let mut s = Self::new();
        for key in slice {
            s.insert(key);
        }
        s
    }

    pub fn from_slice(slice: &[K]) -> Self
    where
        K: Ord + Clone,
    {
        let mut s = Self::new();
        for key in slice {
            s.insert(key.clone());
        }
        s
    }

    pub fn insert(&mut self, key: K) {
        if let Some(cnt) = self.inner.get_mut(&key) {
            *cnt += 1;
        } else {
            self.inner.insert(key, 1);
        }
        self.len += 1;
    }

    pub fn remove(&mut self, key: &K) {
        enum RemoveOption {
            Remove,
            Dec,
            Nothing,
        }
        let op;
        match self.inner.get_mut(key) {
            Some(v) => {
                if *v == 1 {
                    op = RemoveOption::Remove;
                } else {
                    *v -= 1;
                    op = RemoveOption::Dec;
                }
            }
            None => op = RemoveOption::Nothing,
        }

        match op {
            RemoveOption::Nothing => {}
            RemoveOption::Dec => self.len -= 1,
            RemoveOption::Remove => {
                self.inner.remove(key);
                self.len -= 1;
            }
        };
    }

    pub fn peek(&self) -> Option<&K> {
        if let Some((a, _)) = self.inner.first_key_value() {
            return Some(a);
        }
        None
    }

    pub fn pop(&mut self) -> Option<K>
    where
        K: Ord + Clone,
    {
        if let Some((a, _)) = self.inner.first_key_value() {
            let a = a.clone();
            self.remove(&a);
            return Some(a);
        }
        None
    }
}

#[test]
fn t1() {
    let mut a = MultiSet::new();
    for i in (0..10).into_iter().rev() {
        a.insert(i);
    }
    println!("{:?}", a);
    a.remove(&4);
    a.remove(&6);
    println!("{:?}", a);
    println!("{:?}", a.pop());
    println!("{:?}", a.pop());
    println!("{:?}", a);
}

#[test]
fn t2() {
    let mut a = MultiSet::new();
    for i in (0..10).into_iter().map(|v| Reverse(v)).rev() {
        a.insert(i);
    }
    println!("{:?}", a);
    a.remove(&Reverse(4));
    a.remove(&Reverse(6));
    println!("{:?}", a);
    a.pop();
    a.pop();
    println!("{:?}", a);
    a.insert(Reverse(7));
    a.insert(Reverse(7));
    a.insert(Reverse(7));
    println!("{:?}", a);
}
