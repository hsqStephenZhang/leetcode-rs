struct Solution;

impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut candidates = candidates;
        candidates.sort();
        let mut res = vec![];
        let mut cur = vec![];
        let mut used = vec![false; candidates.len()];
        Self::backtrace(target, 0, &mut candidates, &mut cur, &mut used, &mut res);
        res
    }

    pub fn backtrace(
        target: i32,
        index: usize,
        candicates: &mut Vec<i32>,
        cur: &mut Vec<i32>,
        used: &mut Vec<bool>,
        res: &mut Vec<Vec<i32>>,
    ) {
        if target == 0 {
            res.push(cur.iter().map(|&r| r).collect());
            return;
        }
        if index == candicates.len() {
            return;
        }
        Self::backtrace(target, index + 1, candicates, cur, used, res);
        let v = candicates[index];
        if target - v >= 0
            && !(index > 0
                && candicates[index] == candicates[index - 1]
                && used[index - 1] == false)
        {
            cur.push(v);
            used[index] = true;
            Self::backtrace(target - v, index + 1, candicates, cur, used, res);
            used[index] = false;
            cur.pop();
        }
    }
}

#[test]
fn leetcode40_t1() {
    let candicates = vec![10, 1, 2, 7, 6, 1, 5];
    let r = Solution::combination_sum2(candicates, 8);
    println!("{:?}", r);
}

#[test]
fn leetcode40_t2() {
    let candicates = vec![1, 2, 2, 2, 5];
    let r = Solution::combination_sum2(candicates, 5);
    println!("{:?}", r);
}
