struct Solution;

impl Solution {
    pub fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
        let size = card_points.len();
        let window_size = size - k as usize;
        let mut sum: i32 = card_points.iter().take(window_size).sum();
        let mut res = sum;
        for i in window_size..size {
            sum += card_points[i] - card_points[i - window_size];
            res = res.min(sum);
        }
        let total: i32 = card_points.iter().sum();
        total - res
    }
}

#[test]
fn leetcode1423_t1() {
    let card_points = vec![100, 40, 17, 9, 73, 75];
    let k = 3;
    assert_eq!(248, Solution::max_score(card_points, k));
}
