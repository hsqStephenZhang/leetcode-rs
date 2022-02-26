struct Solution;

impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let mut count = s.len();
        if count == 1 {
            return 1;
        }

        for mid in 1..=s.len() - 2 {
            let mut left = (mid - 1) as i32;
            let mut right = mid + 1;
            while left >= 0 && right <= s.len() - 1 {
                if s.chars().nth(left as usize).unwrap() == s.chars().nth(right).unwrap() {
                    count += 1;
                    left -= 1;
                    right += 1;
                } else {
                    break;
                }
            }
        }

        for pair in 0..=s.len() - 2 {
            let mut left = pair as i32;
            let mut right = pair + 1;
            while left >= 0 && right <= s.len() - 1 {
                if s.chars().nth(left as usize).unwrap() == s.chars().nth(right).unwrap() {
                    count += 1;
                    left -= 1;
                    right += 1;
                } else {
                    break;
                }
            }
        }

        count as i32
    }
}

#[test]
fn leetcode647_t1() {
    let s = String::from("a");
    let res = Solution::count_substrings(s);
    assert_eq!(res, 1);
}
