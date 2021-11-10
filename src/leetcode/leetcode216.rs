struct Solution;

impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        let k = k as usize;
        let mut res = vec![];
        let mut used = vec![false; 10];
        let mut path = Vec::with_capacity(k);
        Self::backtrace(k, 0, n, &mut path, &mut used, &mut res);
        res
    }

    fn backtrace(
        k: usize,
        index: usize,
        left: i32,
        path: &mut Vec<i32>,
        used: &mut Vec<bool>,
        res: &mut Vec<Vec<i32>>,
    ) {
        if left < 0 {
            return;
        }
        if index == k {
            if left == 0 {
                res.push(path.clone());
            }
            return;
        }

        let start = *path.last().unwrap_or(&1) as usize;

        for i in start..=9 {
            if used[i] == true {
                continue;
            }

            // 1. not used
            // 2. prune
            if (k - index) * start <= left as usize && (left as usize) <= (k - index) * 9 {
                used[i] = true;
                path.push(i as i32);
                Self::backtrace(k, index + 1, left - i as i32, path, used, res);
                path.pop();
                used[i] = false;
            }
        }
    }
}

#[test]
fn leetcode216_t1() {
    let k = 3;
    let n = 7;
    let r = Solution::combination_sum3(k, n);
    println!("{:?}", r);
}
