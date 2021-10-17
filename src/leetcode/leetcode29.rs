struct Solution;

impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        if dividend == i32::MIN && divisor == -1 {
            return i32::MAX;
        }
        return Self::divide_long(dividend as i64, divisor as i64);
    }

    pub fn divide_long(dividend: i64, divisor: i64) -> i32 {
        let same_symbol = dividend > 0 && divisor > 0 || dividend < 0 && divisor < 0;
        let mut dividend = dividend.abs();
        let divisor = divisor.abs();
        if dividend < divisor {
            return 0;
        }
        let mut res = 0;
        loop {
            let mut cur = divisor;
            let mut adder = 0;
            while cur <= dividend {
                adder += 1;
                cur = cur << 1;
            }
            res += 1 << (adder - 1);
            dividend -= cur >> 1;
            if dividend < divisor {
                break;
            }
        }
        return res * if same_symbol { 1 } else { -1 };
    }
}

#[test]
fn leetcode29_t1() {
    let r = Solution::divide(10, -3);
    assert_eq!(r, -3);

    let r = Solution::divide(10, 2);
    assert_eq!(r, 5);

    let r = Solution::divide(1 << 30, 2);
    assert_eq!(r, 1 << 29);

    let r = Solution::divide(i32::MIN, -1);
    assert_eq!(r, i32::MAX);
}
