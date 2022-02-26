struct Solution0;

// time exceeds limit
impl Solution0 {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let mut coins = coins;
        coins.sort_by(|a, b| b.cmp(a));
        let res = Solution0::inner(&coins, 0, amount);
        return if res == std::i32::MAX { -1 } else { res };
    }

    fn inner(coins: &Vec<i32>, idx: usize, amount: i32) -> i32 {
        if amount == 0 {
            return 0;
        }
        if idx >= coins.len() {
            return i32::MAX;
        }
        let cur = coins[idx];
        if cur > amount {
            return Solution0::inner(coins, idx + 1, amount);
        } else {
            let v1 = Solution0::inner(coins, idx, amount - cur);
            let v2 = Solution0::inner(coins, idx + 1, amount);
            if v1 == i32::MAX {
                return v2;
            } else {
                return (v1 + 1).min(v2);
            }
        }
    }
}

struct Solution;

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let mut dp = vec![amount + 1; amount as usize + 1];
        dp[0] = 0;
        for i in 1..(amount as usize + 1) {
            for &coin in &coins {
                if i as i32 >= coin {
                    dp[i] = dp[i].min(dp[i - coin as usize] + 1);
                }
            }
        }
        return if dp[amount as usize] == amount + 1 {
            -1
        } else {
            dp[dp.len() - 1]
        };
    }
}

#[test]
fn leetcode322_t1() {
    let coins = vec![5, 2, 1];
    let amount = 11;
    // let coins = vec![411, 412, 413, 414, 415, 416, 417, 418, 419, 420, 421, 422];
    // let amount = 9864;
    assert_eq!(Solution::coin_change(coins, amount), 3);
}
