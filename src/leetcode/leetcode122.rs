struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut res = 0;

        for i in 1..prices.len() {
            if prices[i] > prices[i - 1] {
                res += prices[i] - prices[i - 1];
            }
        }

        res
    }
}

#[test]
fn leetcode122_t1() {
    assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 7);
}
