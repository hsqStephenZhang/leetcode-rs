struct Solution;

impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let target = s.as_str();
        let mut result = Vec::with_capacity(3);
        let mut total = Vec::new();
        Self::inner(target, 0, &mut result, &mut total);
        return total;
    }

    fn inner(s: &str, index: usize, spliter: &mut Vec<usize>, total: &mut Vec<String>) {
        if spliter.len() == 3 {
            if index < s.len() && Self::is_valid(&s[index..]) {
                total.push(Self::spliter_to_string(s, spliter));
            }
            //dbg!(total.clone());
            return;
        }

        let mut next = index + 1;
        while next <= s.len() && Self::is_valid(&s[index..next]) {
            spliter.push(next);
            //dbg!(next, spliter.clone());
            Self::inner(s, next, spliter, total);
            spliter.pop();
            next += 1;
        }
    }

    #[inline]
    fn spliter_to_string(s: &str, spliter: &Vec<usize>) -> String {
        debug_assert_eq!(spliter.len(), 3);

        let s1 = &s[0..spliter[0]];
        let s2 = &s[spliter[0]..spliter[1]];
        let s3 = &s[spliter[1]..spliter[2]];
        let s4 = &s[spliter[2]..];
        let r = vec![s1, s2, s3, s4].join(".");
        r
    }

    #[inline(always)]
    fn is_valid(s: &str) -> bool {
        if s.starts_with("0") && s.len() != 1 {
            return false;
        }
        let r = s.parse::<usize>();
        match r {
            Err(_) => false,
            Ok(val) => val <= 255,
        }
    }
}

#[test]
fn leetcode93_t1() {
    let s = "101023";
    let r = Solution::restore_ip_addresses(s.into());
    println!("{:?}", r);
}
