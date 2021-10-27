struct Solution;

impl Solution {
    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        if matrix.len() == 0 || matrix[0].len() == 0 {
            return 0;
        }
        let n = matrix.len();
        let m = matrix[0].len();
        let mut dp = vec![vec![0; m]; n];

        let mut res = 0;

        for row in 0..n {
            for col in 0..m {
                res = res.max(Self::dfs(&matrix, row, col, &mut dp));
            }
        }

        res as i32
    }

    fn dfs(matrix: &Vec<Vec<i32>>, row: usize, col: usize, dp: &mut Vec<Vec<usize>>) -> usize {
        if dp[row][col] != 0 {
            return dp[row][col];
        }
        let n = matrix.len() as i32;
        let m = matrix[0].len() as i32;
        dp[row][col] += 1;
        const STEPS: [[i32; 2]; 4] = [[0, 1], [0, -1], [1, 0], [-1, 0]];

        for step in STEPS {
            let i = row as i32 + step[0];
            let j = col as i32 + step[1];
            if i >= 0
                && i < n
                && j >= 0
                && j < m
                && matrix[i as usize][j as usize] > matrix[row][col]
            {
                dp[row][col] = dp[row][col].max(Self::dfs(matrix, i as usize, j as usize, dp) + 1);
            }
        }
        dp[row][col]
    }
}

#[test]
fn leetcode329_t1() {
    let matrix = vec![vec![9, 9, 4], vec![6, 6, 8], vec![2, 1, 1]];
    let r = Solution::longest_increasing_path(matrix);
    println!("{:?}", r);
}
