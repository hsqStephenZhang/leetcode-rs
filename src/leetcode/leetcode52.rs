struct Solution;

impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        let n = n as usize;
        let mut used = vec![false; n];
        let mut path = Vec::with_capacity(n);
        let mut res = 0;

        Self::backtrace(n, &mut used, &mut path, &mut res);

        res
    }

    fn backtrace(
        n: usize,
        used: &mut Vec<bool>,
        path: &mut Vec<usize>,
        res: &mut i32,
    ) {
        if path.len() == used.len() {
            *res += 1;
            return;
        }
        // col 对应列号
        for col in 0..n {
            if used[col] == true {
                continue;
            }

            if Self::validate(col, path) {
                used[col] = true;
                path.push(col);
                Self::backtrace(n, used, path, res);
                path.pop();
                used[col] = false;
            }
        }
    }

    fn validate(col: usize, path: &Vec<usize>) -> bool {
        let row = path.len();
        for (row0, &col0) in path.iter().enumerate() {
            let delta = if col > col0 { col - col0 } else { col0 - col };
            if (row - row0).eq(&delta) {
                return false;
            }
        }

        true
    }
}

#[test]
fn leetcode52_t1() {
    let r = Solution::total_n_queens(8);
    println!("{:?}", r);

    // let a: Vec<usize> = vec![1, 3, 0, 2];

    // println!("{:?}", r);
}
