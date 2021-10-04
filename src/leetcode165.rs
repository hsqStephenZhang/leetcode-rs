#[cfg(test)]
use std::str::FromStr;

struct Solution;

impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let v1 = version1.split(".").collect::<Vec<&str>>();
        let v2 = version2.split(".").collect::<Vec<&str>>();
        for versions in v1.iter().zip(v2.iter()) {
            let v1 = versions.0.parse::<usize>().unwrap();
            let v2 = versions.1.parse::<usize>().unwrap();
            dbg!(v1, v2);
            if v1 > v2 {
                return 1;
            } else if v1 < v2 {
                return -1;
            }
        }
        let delta_len = v1.len() as i32 - v2.len() as i32;
        let remained = if delta_len > 0 {
            v1.iter().skip(v2.len())
        } else {
            v2.iter().skip(v1.len())
        };
        for r in remained {
            let v = r.parse::<usize>().unwrap();
            if v > 0 {
                return if delta_len > 0 { 1 } else { -1 };
            }
        }
        return 0;
    }
}

#[test]
fn leetcode165_t1() {
    //let s1 = String::from_str("1.001.1.0").unwrap();
    //let s2 = String::from_str("1.001.1").unwrap();
    // let r = Solution::compare_version(s1, s2);
    // assert_eq!(r, 0);

    // let s1 = String::from_str("1.001.1.1").unwrap();
    // let s2 = String::from_str("1.001.1").unwrap();
    // let r = Solution::compare_version(s1, s2);
    // assert_eq!(r, 1);

    let s1 = String::from_str("0.1").unwrap();
    let s2 = String::from_str("1.1").unwrap();
    let r = Solution::compare_version(s1, s2);
    assert_eq!(r, -1);
}
