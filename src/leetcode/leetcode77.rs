struct Solution;

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut path = vec![];
        let n = n as usize;
        let k = k as usize;
        Self::backtrace(0, n, k, &mut path, &mut res);
        res
    }

    pub fn backtrace(
        index: usize,
        n: usize,
        k: usize,
        path: &mut Vec<i32>,
        res: &mut Vec<Vec<i32>>,
    ) {
        if k == 0 {
            res.push(path.clone());
            return;
        }
        if n - index < k {
            return;
        }
        for j in index..n {
            path.push((j + 1) as i32);
            Self::backtrace(j + 1, n, k - 1, path, res);
            path.pop();
        }
    }
}

#[test]
fn leetcode77_t1() {
    let r = Solution::combine(4, 2);
    println!("{:?}", r);
}
