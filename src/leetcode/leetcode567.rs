use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let n = s1.len();
        let mut total = 0;
        let mut m = HashMap::<u8, i32>::new();
        for c in s1.as_bytes().iter() {
            match m.get(c) {
                None => {
                    total += 1;
                    m.insert(*c, 1);
                }
                Some(&val) => {
                    m.insert(*c, val + 1);
                }
            }
        }
        for c in s2.as_bytes().iter().take(n) {
            if let Some(&val) = m.get(c) {
                if val == 1 {
                    total -= 1;
                }
                m.insert(*c, val - 1);
            }
        }
        if total == 0 {
            return true;
        }

        dbg!(total);

        for (i, c) in s2.as_bytes().iter().enumerate().skip(n) {
            let prev_c = s2.as_bytes()[i - n];
            if let Some(&val) = m.get(c) {
                if val == 1 {
                    total -= 1;
                }
                m.insert(*c, val - 1);
            }
            if let Some(&val) = m.get(&prev_c) {
                if val == 0 {
                    total += 1;
                }
                m.insert(prev_c, val + 1);
            }
            dbg!(m.clone(), *c, prev_c, total);
            if total == 0 {
                return true;
            }
        }
        false
    }
}

// we can use array to mock the hashmap,
// it's performance is much better than the real map
struct Solution2;

type Item = Option<i32>;

impl Solution2 {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let n = s1.len();
        let mut m: [Item; 26] = [None; 26];
        let mut total = 0;
        for &c in s1.as_bytes().iter() {
            let index = idx(c);
            match m[index] {
                None => {
                    total += 1;
                    m[index] = Some(1);
                }
                Some(val) => {
                    m[index] = Some(val + 1);
                }
            }
        }
        for &c in s2.as_bytes().iter().take(n) {
            let index = idx(c);
            if let Some(val) = m[index] {
                if val == 1 {
                    total -= 1;
                }
                m[index] = Some(val - 1);
            }
        }
        if total == 0 {
            return true;
        }

        for (i, &c) in s2.as_bytes().iter().enumerate().skip(n) {
            let prev_c = s2.as_bytes()[i - n];
            let index1 = idx(c);
            let index2 = idx(prev_c);
            if let Some(val) = m[index1] {
                if val == 1 {
                    total -= 1;
                }
                m[index1] = Some(val - 1);
            }
            if let Some(val) = m[index2] {
                if val == 0 {
                    total += 1;
                }
                m[index2] = Some(val + 1);
            }
            if total == 0 {
                return true;
            }
        }
        false
    }
}

#[inline(always)]
pub fn idx(c: u8) -> usize {
    (c - b'a') as usize
}

#[test]
fn leetcode567_t1() {
    let s1 = "abc".into();
    let s2 = "ccbbaa".into();
    let r = Solution::check_inclusion(s1, s2);
    println!("{}", r);
}
