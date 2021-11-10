use std::collections::BTreeSet;

struct Solution;

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let mut res = 0;
        let chars = s.as_bytes().iter().map(|&c| c).collect::<BTreeSet<u8>>();
        for current_char in chars {
            let mut remained = k;
            let mut l = 0;
            let mut tmp = 0;
            for (right, &c) in s.as_bytes().iter().enumerate() {
                if c != current_char {
                    remained -= 1;
                }
                if remained >= 0 {
                    tmp = tmp.max(right - l + 1);
                } else if remained == -1 {
                    loop {
                        if s.as_bytes()[l] != current_char {
                            remained += 1;
                            l += 1;
                            break;
                        } else {
                            l += 1;
                        }
                    }
                } else if remained < -1 {
                    unreachable!("remained is bigger than -2");
                }
            }

            res = res.max(tmp);
        }
        res as i32
    }
}

#[test]
fn leetcode424_t1() {
    let mut s = String::from("ABABBBCCCBCCCBCC");
    let r = Solution::character_replacement(s, 2);
    println!("{:?}", r);
}
