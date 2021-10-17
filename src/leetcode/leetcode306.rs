use std::cmp::Ordering;

struct Solution;

#[warn(unused_assignments)]
impl Solution {
    pub fn is_additive_number(num: String) -> bool {
        let s = num.as_bytes();
        if s.len() < 3 {
            return false;
        }
        let size = s.len();
        let mut left = 1;
        let mut right = left;
        dbg!(left, right);
        while left <= size - 2 {
            let first = &s[..left];
            if first.starts_with("0".as_bytes()) && first.len() > 1 {
                break;
            }
            right = left + 1;
            while right < size {
                let second = &s[left..right];
                if second.starts_with("0".as_bytes()) && second.len() > 1 {
                    break;
                }
                match Self::second_stage(&s[right..], first, second) {
                    None => {}
                    Some(r) => {
                        if r {
                            return true;
                        }
                    }
                }
                right += 1;
            }
            left += 1;
        }
        false
    }

    fn second_stage(s: &[u8], v1: &[u8], v2: &[u8]) -> Option<bool> {
        if s.is_empty() {
            return None;
        }
        if s.starts_with("0".as_bytes()) && !(v1.eq("0".as_bytes()) && v2.eq("0".as_bytes())) {
            return Some(false);
        }
        let size = s.len();
        let mut first = 1;
        while first <= size {
            let r = s[..first]
                .iter()
                .cmp(Self::add_str(v1, v2).as_bytes().iter());
            if let Ordering::Equal = r {
                match Self::second_stage(&s[first..], v2, &s[..first]) {
                    None => return Some(true),
                    Some(v) => {
                        if v {
                            return Some(true);
                        }
                    }
                }
            }
            first += 1;
        }
        Some(false)
    }

    #[inline(always)]
    fn add_str(num1: &[u8], num2: &[u8]) -> String {
        let (mut num1, mut num2) = (num1.iter().rev(), num2.iter().rev());
        let mut ret = vec![];
        let mut carry = 0;
        loop {
            match (num1.next(), num2.next()) {
                (None, None) => break,
                (a, b) => {
                    let mut n = a.unwrap_or(&b'0') + b.unwrap_or(&b'0') - b'0' + carry;
                    carry = 0;
                    if n > b'9' {
                        n -= 10;
                        carry = 1;
                    }
                    ret.push(n as char);
                }
            }
        }
        if carry != 0 {
            ret.push('1');
        }
        ret.iter().rev().collect()
    }
}

#[test]
fn leetcode306_t1() {
    let s = "11111111111011111111111".into();
    let r = Solution::is_additive_number(s);
    assert_eq!(r, true);

    let s = "12358".into();
    let r = Solution::is_additive_number(s);
    assert_eq!(r, true);

    let s = "113".into();
    let r = Solution::is_additive_number(s);
    assert_eq!(r, false);
    // let r = "11111111111011111111111".parse::<i32>().unwrap();
}
