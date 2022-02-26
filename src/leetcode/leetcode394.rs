struct Solution;

impl Solution {
    pub fn decode_string(s: String) -> String {
        return Solution::inner(&s);
    }

    fn inner(s: &str) -> String {
        let res = String::new();
        let location = first_number(s);
        match location {
            Some((left_idx, right_idx, repeat)) => {
                let mut last_closet_idx = right_idx + 2;
                let mut cnt = 1;
                while cnt > 0 {
                    if s.as_bytes()[last_closet_idx] == b'[' {
                        cnt += 1;
                    } else if s.as_bytes()[last_closet_idx] == b']' {
                        cnt -= 1;
                    }
                    last_closet_idx += 1;
                }
                last_closet_idx -= 1;
                let substr = &s[(right_idx + 2)..last_closet_idx];
                let subres = Solution::inner(substr);
                let mut res = String::with_capacity(
                    subres.len() * repeat as usize + s.len() - last_closet_idx - 1,
                );
                res.push_str(&Solution::inner(&s[0..left_idx]));
                for _ in 0..repeat as usize {
                    res.push_str(&subres);
                }
                res.push_str(&Solution::inner(&s[(last_closet_idx + 1)..]));
                return res;
            }
            None => return s.into(),
        }
    }
}

// first positive number in a string
fn first_number(s: &str) -> Option<(usize, usize, usize)> {
    let location = s
        .as_bytes()
        .iter()
        .enumerate()
        .find(|&(i, &c)| c >= b'0' && c <= b'9');
    match location {
        Some((left_idx, number_ch)) => {
            let mut right_idx = left_idx;
            let mut number = (number_ch - b'0') as usize;
            for &c in s.as_bytes().iter().skip(left_idx + 1) {
                if c >= b'0' && c <= b'9' {
                    right_idx += 1;
                    number = number * 10 + (c - b'0') as usize;
                } else {
                    break;
                }
            }
            return Some((left_idx, right_idx, number));
        }
        None => return None,
    }
}

#[test]
fn leetcode394_t1() {
    let s = "3[a]2[bc]".to_string();
    assert_eq!(Solution::decode_string(s), "aaabcbc".to_string());
    let s = "2[abc]3[cd]ef".to_string();
    assert_eq!(Solution::decode_string(s), "abcabccdcdcdef".to_string());
}
