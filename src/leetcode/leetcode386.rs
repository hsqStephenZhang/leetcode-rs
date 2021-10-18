struct Solution;

impl Solution {
    pub fn lexical_order(n: i32) -> Vec<i32> {
        let mut res = Vec::with_capacity(n as usize);
        for val in (1..=9).into_iter() {
            if val > n {
                continue;
            }
            res.push(val);
            Self::dfs(10 * val, n, &mut res);
        }
        res
    }

    pub fn dfs(v: i32, n: i32, res: &mut Vec<i32>) {
        for val in (v..(v + 10)).into_iter() {
            if val > n {
                return;
            }
            res.push(val);
            Self::dfs(10 * val, n, res);
        }
    }
}

// WARN: copied solution
struct Solution2;

impl Solution2 {
    pub fn lexical_order(n: i32) -> Vec<i32> {
        let mut res = Vec::with_capacity(n as usize);
        let mut num = 1;

        // mock the recursion stack
        while res.len() < n as usize {
            // call into the next layer
            while num <= n {
                res.push(num);
                num *= 10;
            }
            // return back to the previous layer
            while num % 10 == 9 || num > n {
                num /= 10;
            }
            num += 1;
        }

        res
    }
}

#[test]
fn leetcode396_t1() {
    let r1 = Solution::lexical_order(13);
    let r2 = Solution2::lexical_order(13);
    println!("{:?}", r1);
    println!("{:?}", r2);
}
