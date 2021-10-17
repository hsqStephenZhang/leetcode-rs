struct Solution;

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut res = vec![0; temperatures.len()];
        let mut stack: Vec<(usize, i32)> = Vec::with_capacity(temperatures.len());

        for (i, &val) in temperatures.iter().enumerate() {
            while match stack.last() {
                None => false,
                Some(&(_, top_val)) => top_val < val,
            } {
                let (j, _) = stack.pop().unwrap();
                res[j] = (i - j) as i32;
            }
            stack.push((i, val));
            // println!("i:{},{:?},{:?}", i, res.clone(), stack.clone());
        }

        res
    }
}

#[test]
fn leetcode739_t1() {
    let temperatures = vec![73, 74, 75, 71, 69, 72, 76, 73];
    let r = Solution::daily_temperatures(temperatures);
    assert_eq!(r, vec![1, 1, 4, 2, 1, 1, 0, 0]);
}
