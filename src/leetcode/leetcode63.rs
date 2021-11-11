struct Solution;

impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let m = obstacle_grid.len();
        let n = obstacle_grid[0].len();

        let mut level1 = vec![0; n];
        let mut level2 = vec![0; n];

        // init level1
        for (index, &val) in obstacle_grid[0].iter().enumerate() {
            if val == 0 {
                level1[index] = 1;
            } else {
                break;
            }
        }

        for i in 1..m {
            // 如果 (i,0) 位置不是 0，赋值为 level1[0]
            level2[0] = if obstacle_grid[i][0] == 0 {
                level1[0]
            } else {
                0
            };
            for j in 1..n {
                level2[j] = if obstacle_grid[i][j] == 0 {
                    level2[j - 1] + level1[j]
                } else {
                    0
                };
            }

            std::mem::swap(&mut level1, &mut level2);
        }

        level1[n - 1]
    }
}

#[test]
fn leetcode63_t1() {
    let nums = vec![vec![0, 0, 0], vec![1, 1, 1], vec![0, 0, 0]];
    let r = Solution::unique_paths_with_obstacles(nums);
    println!("{}", r);
}