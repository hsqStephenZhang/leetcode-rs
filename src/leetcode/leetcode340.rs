use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn length_of_longest_substring_k_distinct(s: String, k: i32) -> i32 {
        let mut res = 0;
        let mut total = 0;
        let mut count: HashMap<u8, i32> = HashMap::new();
        let mut left = 0;

        let s = s.as_bytes();

        for (right, v) in s.iter().enumerate() {
            if let Some(t) = count.get_mut(&v) {
                *t += 1;
            } else {
                count.insert(*v, 1);
                total += 1;
            }

            if total <= k {
                res = res.max(right - left + 1);
            } else if total == k + 1 {
                while let Some(left_v) = count.get_mut(&s[left]) {
                    if *left_v == 1 {
                        count.remove(&s[left]);
                        res = res.max(right - left);
                        left += 1;
                        total = k;
                        break;
                    } else {
                        *left_v -= 1;
                        left += 1;
                    }
                }
            } else if total > k + 1 {
                unreachable!("corrupt program");
            }
        }

        res as i32
    }
}

#[test]
fn leetcode340_t1() {
    let s = String::from("abccddeeff");
    let r = Solution::length_of_longest_substring_k_distinct(s,3);
    println!("{:?}", r);
}
