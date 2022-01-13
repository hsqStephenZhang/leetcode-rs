struct Solution;

// time limit exceeded
impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        let m = board.len();
        let n = board[0].len();
        let mut searched = vec![vec![false; n]; m];
        let is_valid =
            |i: i32, j: i32| -> bool { i >= 0 && j >= 0 && (i < m as i32) && (j < n as i32) };

        for i in 0..m {
            for j in 0..n {
                if board[i][j] == 'O' && !searched[i][j] {
                    let mut queue = Vec::with_capacity(m * n / 2);
                    queue.push([i, j]);
                    let mut index = 0;
                    let mut is_surround = true;
                    while index < queue.len() {
                        let cur_i = queue[index][0];
                        let cur_j = queue[index][1];
                        searched[cur_i][cur_j] = true;
                        for step in [[1, 0], [-1, 0], [0, 1], [0, -1]].iter() {
                            let next_i = cur_i as i32 + step[0];
                            let next_j = cur_j as i32 + step[1];
                            if is_valid(next_i, next_j)
                                && board[next_i as usize][next_j as usize] == 'O'
                                && !searched[next_i as usize][next_j as usize]
                            {
                                queue.push([next_i as usize, next_j as usize])
                            }
                        }

                        if cur_i == 0 || cur_j == 0 || cur_i == m - 1 || cur_j == n - 1 {
                            is_surround = false;
                        }

                        index += 1;
                    }

                    if is_surround {
                        for idx in queue {
                            board[idx[0]][idx[1]] = 'X';
                        }
                    }
                }
            }
        }
    }
}

struct Solution2;

impl Solution2 {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        fn dfs(
            x: usize,
            y: usize,
            lenx: usize,
            leny: usize,
            board: &Vec<Vec<char>>,
            searched: &mut Vec<Vec<bool>>,
        ) {
            if x > 0 && board[x - 1][y] == 'O' && !searched[x - 1][y] {
                searched[x - 1][y] = true;
                dfs(x - 1, y, lenx, leny, board, searched);
            }
            if x + 1 < lenx && board[x + 1][y] == 'O' && !searched[x + 1][y] {
                searched[x + 1][y] = true;
                dfs(x + 1, y, lenx, leny, board, searched);
            }
            if y > 0 && board[x][y - 1] == 'O' && !searched[x][y - 1] {
                searched[x][y - 1] = true;
                dfs(x, y - 1, lenx, leny, board, searched);
            }

            if y + 1 < leny && board[x][y + 1] == 'O' && !searched[x][y + 1] {
                searched[x][y + 1] = true;
                dfs(x, y + 1, lenx, leny, board, searched);
            }
        }

        let lenx = board.len();
        let leny = board[0].len();
        let search_x = vec![false; leny];
        let mut search = vec![search_x.clone(); lenx];

        // four edge line
        for i in 0..lenx {
            if board[i][0] == 'O' && !search[i][0] {
                search[i][0] = true;
                dfs(i, 0, lenx, leny, board, &mut search);
            }
            if board[i][leny - 1] == 'O' && !search[i][leny - 1] {
                search[i][leny - 1] = true;
                dfs(i, leny - 1, lenx, leny, board, &mut search);
            }
        }
        for j in 0..leny {
            if board[0][j] == 'O' && !search[0][j] {
                search[0][j] = true;
                dfs(0, j, lenx, leny, board, &mut search);
            }
            if board[lenx - 1][j] == 'O' && !search[lenx - 1][j] {
                search[lenx - 1][j] = true;
                dfs(lenx - 1, j, lenx, leny, board, &mut search);
            }
        }

        for i in 1..lenx - 1 {
            for j in 1..leny - 1 {
                if board[i][j] == 'O' && !search[i][j] {
                    board[i][j] = 'X';
                }
            }
        }
    }
}

#[test]
fn leetcode130_t1() {
    let mut board = vec![
        vec!['X', 'X', 'X', 'X'],
        vec!['X', 'O', 'O', 'X'],
        vec!['X', 'X', 'O', 'X'],
        vec!['X', 'O', 'X', 'X'],
    ];
    Solution::solve(&mut board);
    dbg!(board);
}
