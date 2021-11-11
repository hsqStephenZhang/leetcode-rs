struct Solution;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        if m == 1 {
            return 1;
        }

        let m = m as usize;
        let n = n as usize;

        let mut level1 = vec![1; n];
        let mut level2 = vec![0; n];

        for _ in 0..m - 1 {
            level2[0] = level1[0];
            for j in 1..n {
                level2[j] = level2[j - 1] + level1[j];
            }

            std::mem::swap(&mut level1, &mut level2);
        }

        level1[n - 1]
    }
}

#[test]
fn leetcode62_t1() {
    let r = Solution::unique_paths(3, 7);
    println!("{}", r);

    let r = Solution::unique_paths(3, 2);
    println!("{}", r);
}