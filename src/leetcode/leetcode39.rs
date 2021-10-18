struct Solution;

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut candidates = candidates;
        let mut res = vec![];
        let mut cur = vec![];
        Self::backtrace(target, 0, &mut candidates, &mut cur, &mut res);
        res
    }

    pub fn backtrace(
        target: i32,
        index: usize,
        candicates: &mut Vec<i32>,
        cur: &mut Vec<i32>,
        res: &mut Vec<Vec<i32>>,
    ) {
        if index == candicates.len() {
            return;
        }
        if target == 0 {
            res.push(cur.iter().map(|&r| r).collect());
            return;
        }
        Self::backtrace(target, index + 1, candicates, cur, res);
        let v = candicates[index];
        if target - v >= 0 {
            cur.push(v);
            Self::backtrace(target - v, index, candicates, cur, res);
            cur.pop();
        }
    }
}

#[test]
fn leetcode39_t1() {
    let candicates = vec![2, 3, 6, 7];
    let r = Solution::combination_sum(candicates, 7);
    println!("{:?}", r);
}
