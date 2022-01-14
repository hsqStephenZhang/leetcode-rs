struct Solution;

impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let mut ans = vec![];
        let (n, m) = (s.len(), p.len());
        let mut count_s = [0_usize; 26];
        let mut count_p = [0_usize; 26];
        let mut total_kinds = 0;
        for c in p.into_bytes() {
            count_s[c as usize - b'a' as usize] += 1;
            if count_s[c as usize - b'a' as usize] == 1 {
                total_kinds += 1;
            }
        }
        let mut matched_kinds = 0;
        let s = s.into_bytes();
        for i in 0..(m - 1).min(n) {
            count_p[s[i] as usize - b'a' as usize] += 1;
            if count_p[s[i] as usize - b'a' as usize] == count_s[s[i] as usize - b'a' as usize] {
                matched_kinds += 1;
            } else if count_p[s[i] as usize - b'a' as usize]
                == count_s[s[i] as usize - b'a' as usize] + 1
            {
                matched_kinds -= 1;
            }
        }
        for i in m - 1..n {
            count_p[s[i] as usize - b'a' as usize] += 1;
            if count_p[s[i] as usize - b'a' as usize] == count_s[s[i] as usize - b'a' as usize] {
                matched_kinds += 1;
            } else if count_p[s[i] as usize - b'a' as usize]
                == count_s[s[i] as usize - b'a' as usize] + 1
            {
                matched_kinds -= 1;
            }
            if matched_kinds == total_kinds {
                ans.push((i + 1 - m) as i32)
            }
            count_p[s[i + 1 - m] as usize - b'a' as usize] -= 1;
            if count_p[s[i + 1 - m] as usize - b'a' as usize]
                == count_s[s[i + 1 - m] as usize - b'a' as usize]
            {
                matched_kinds += 1;
            } else if count_p[s[i + 1 - m] as usize - b'a' as usize]
                == count_s[s[i + 1 - m] as usize - b'a' as usize] - 1
            {
                matched_kinds -= 1;
            }
        }
        ans
    }
}

#[test]
fn leetcode438_t1() {
    let s = "abab";
    let p = "ab";
    let r = Solution::find_anagrams(s.into(), p.into());
    dbg!(r);
}
