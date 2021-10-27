struct Solution;

impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = Vec::with_capacity(intervals.len() >> 1);
        let mut intervals = intervals;
        intervals.sort_by(|x, y| {
            let x = x[0];
            let y = y[0];
            x.cmp(&y)
        });

        for v in intervals.into_iter() {
            if res.len() == 0 || v[0] > res.last().unwrap()[1] {
                res.push(v);
            } else {
                let l = res.last_mut().unwrap();
                l[1] = l[1].max(v[1]);
            }
        }

        res
    }
}

#[test]
fn leetcode56_t1() {
    let nums = vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]];
    let r = Solution::merge(nums);
    println!("{:?}", r);

    let nums = vec![vec![1, 4], vec![4, 5]];
    let r = Solution::merge(nums);
    println!("{:?}", r);
}
