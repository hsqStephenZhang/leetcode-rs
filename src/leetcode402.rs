use std::str::FromStr;

struct Solution;

impl Solution {
    pub fn remove_kdigits(num: String, k: i32) -> String {
        let mut k = k;
        let remained = num.len() - k as usize;
        let mut stack = Vec::new();
        // iter over the num,
        num.as_bytes()
            .into_iter()
            .map(|&val| {
                // 2 conditions are required to keep the monotonous stack
                //   1) k>0
                //   2) the stack is not empty and the top of stack if greater than the current number
                while k > 0
                    && match stack.last() {
                        None => false,
                        Some(&top_val) => dbg!(top_val > val),
                    }
                {
                    stack.pop();
                    k -= 1;
                }
                stack.push(val);
            })
            .count();

        // remove the prefix '0'
        let r = &stack[..remained]
            .into_iter()
            .skip_while(|&&b| b == b'0')
            .map(|&x| x)
            .collect::<Vec<u8>>();
        let r = String::from_utf8(r.clone()).unwrap();
        // in case the left string is empty, then we will return "0"
        return if r.len() == 0 { "0".into() } else { r };
    }
}

#[test]
fn leetcode402_t1() {
    let num = "1432219".into();
    let k = 3;
    let r = Solution::remove_kdigits(num, k);
    println!("result:{}", r);
}
