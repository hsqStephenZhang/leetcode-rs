struct Solution;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        Solution::inner(
            &matrix,
            target,
            (0, 0),
            (matrix.len() - 1, matrix[0].len() - 1),
        )
    }

    fn inner(
        matrix: &Vec<Vec<i32>>,
        target: i32,
        top_left: (usize, usize),
        bottom_right: (usize, usize),
    ) -> bool {
        let height = bottom_right.0 - top_left.0 + 1;
        let width = bottom_right.1 - top_left.1 + 1;
        if height <= 2 && width <= 2 {
            for i in top_left.0..=bottom_right.0 {
                for j in top_left.1..=bottom_right.1 {
                    if matrix[i][j] == target {
                        return true;
                    }
                }
            }
            return false;
        } else if height <= 2 {
            for line_idx in top_left.0..=bottom_right.0 {
                if matrix[line_idx][top_left.1..=bottom_right.1]
                    .binary_search_by(|&x| x.cmp(&target))
                    .is_ok()
                {
                    return true;
                }
            }
            return false;
        } else if width <= 2 {
            for col_indx in top_left.1..=bottom_right.1 {
                if matrix
                    .binary_search_by(|x| x[col_indx].cmp(&target))
                    .is_ok()
                {
                    return true;
                }
            }
            return false;
        }

        let mid_m = (top_left.0 + bottom_right.0) >> 1;
        let mid_n = (top_left.1 + bottom_right.1) >> 1;
        if matrix[mid_m][mid_n] == target {
            return true;
        } else if matrix[mid_m][mid_n] > target {
            return Solution::inner(matrix, target, top_left, (mid_m, mid_n))
                | Solution::inner(
                    matrix,
                    target,
                    (top_left.0, mid_n + 1),
                    (mid_m - 1, bottom_right.1),
                )
                | Solution::inner(
                    matrix,
                    target,
                    (mid_m + 1, top_left.1),
                    (bottom_right.0, mid_n - 1),
                );
        } else {
            return Solution::inner(matrix, target, (mid_m, mid_n), bottom_right)
                | Solution::inner(
                    matrix,
                    target,
                    (top_left.0, mid_n + 1),
                    (mid_m - 1, bottom_right.1),
                )
                | Solution::inner(
                    matrix,
                    target,
                    (mid_m + 1, top_left.1),
                    (bottom_right.0, mid_n - 1),
                );
        }
        false
    }
}

#[test]
fn leetcode240_t1() {
    let mut matrix = vec![
        vec![1, 4, 7, 11, 15],
        vec![2, 5, 8, 12, 19],
        vec![3, 6, 9, 16, 22],
        vec![10, 13, 14, 17, 24],
        vec![18, 21, 23, 26, 30],
    ];
    for line in matrix.iter() {
        for &val in line.iter() {
            let res = Solution::search_matrix(matrix.clone(), val);
            assert_eq!(res, true);
        }
    }
}

#[test]
fn leetcode240_t2() {
    /*
     * 4  6  9  10 15
     * 9  12 13 15 16
     */
    let matrix = vec![vec![4, 6, 9, 10, 15], vec![9, 12, 13, 15, 16]];
    let res = Solution::search_matrix(matrix, 6);
    assert_eq!(res, false);
}
