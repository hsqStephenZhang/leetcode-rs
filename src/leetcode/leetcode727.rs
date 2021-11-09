struct Solution;

struct Result {
    left: usize,
    length: usize,
}

impl Result {
    pub fn new() -> Self {
        Self {
            left: 0,
            length: usize::MAX,
        }
    }

    pub fn update(&mut self, left: usize, length: usize) {
        if length < self.length {
            self.length = length;
            self.left = left;
        }
    }
}

impl Solution {
    pub fn min_window(s1: String, s2: String) -> String {
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let len1 = s1.len();
        let len2 = s2.len();

        let mut res = Result::new();

        let starts = s1.iter().enumerate().filter(|&(_, &c)| c == s2[0]).map(|(index, _)| index).collect::<Vec<usize>>();
        for start in starts {
            let iter = s1.iter().enumerate().skip(start);
            let mut count = [0; 256];
            let mut matched = 0;
            let mut left = start;
            for (i, &c) in iter {
                count[c as usize] += 1;
                if c == s2[matched] {
                    matched += 1;
                }
                if matched == len2 {
                    res.update(left, i - left + 1);

                    while left < i {
                        if s1[left] != s2[0] {
                            left += 1;
                            continue;
                        } else {
                            if count[s2[0] as usize] == 1 {
                                res.update(left, i - left + 1);
                                break;
                            } else {
                                left += 1;
                            }
                        }
                    }
                    break;
                }
            }
        }
        if res.length == usize::MAX {
            return "".into();
        }
        return String::from_utf8_lossy(&s1[res.left..(res.left + res.length)]).into_owned();
    }
}

#[test]
fn leetcode727_t1() {
    // "jmeqksfrsdcmsiwvaovztaqenprpvnbstl"
    // "u"
    let s = String::from("jmeqksfrsdcmsiwvaovztaqenprpvnbstl");
    let t = String::from("u");
    let r = Solution::min_window(s, t);
    println!("{}", r);
}