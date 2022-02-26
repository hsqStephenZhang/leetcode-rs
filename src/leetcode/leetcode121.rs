struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut stack = vec![];
        let mut max_profit = 0;
        for &val in prices.iter() {
            if stack.len() == 0 {
                stack.push(val);
            } else if stack.len() == 1 {
                if val > stack[0] {
                    stack.push(val);
                } else {
                    stack[0] = val;
                }
            } else if stack.len() == 2 {
                if val > stack[1] {
                    stack[1] = val;
                } else if val < stack[0] {
                    max_profit = max_profit.max(stack[1] - stack[0]);
                    stack[0] = val;
                    stack.pop();
                }
            }
        }
        return if stack.len() == 2 {
            max_profit.max(stack[1] - stack[0])
        } else {
            max_profit
        };
    }
}

#[test]
fn leetcode121_t1() {
    let prices = vec![2, 4, 1];
    let res = Solution::max_profit(prices);
    assert_eq!(res, 2);
}
