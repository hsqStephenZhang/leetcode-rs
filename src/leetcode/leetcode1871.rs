struct Solution;

impl Solution {
    pub fn can_reach(s: String, min_jump: i32, max_jump: i32) -> bool {
        let v = s.as_bytes();
        let size = v.len();
        let mut prefix = vec![0; size];
        let mut dp = vec![0; size];

        dp[0] = 1;
        prefix[0] = 1;
        ((min_jump as usize)..=(max_jump as usize))
            .into_iter()
            .filter(|&i| i < size && v[i] == b'0')
            .map(|i| {
                dp[i] = 1;
            })
            .count();

        for i in (1..=(max_jump as usize)).take_while(|&i| i < size) {
            prefix[i] = prefix[i - 1] + dp[i];
        }

        for i in (min_jump as usize)..size {
            let left = i as i32 - 1 - max_jump as i32;
            let right = i as i32 - min_jump as i32;
            if v[i] == b'0' {
                let left_val = if left < 0 { 0 } else { prefix[left as usize] };
                let right_val = prefix[right as usize];
                let total = right_val - left_val;

                dp[i] = if total == 0 { 0 } else { 1 };
            }
            prefix[i] = prefix[i - 1] + dp[i];
        }

        dp[size - 1] == 1
    }
}

#[test]
fn leetcode1871_t1() {
    let s = "000100100010".into();
    let min_jump = 1;
    let max_jump = 2;
    let r = Solution::can_reach(s, min_jump, max_jump);
    println!("r:{}", r);
}

#[test]
fn leetcode1871_t2() {
    let s = "011010".into();
    let min_jump = 2;
    let max_jump = 3;
    let r = Solution::can_reach(s, min_jump, max_jump);
    println!("r:{}", r);
}
