struct Solution;

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut res: Vec<Vec<i32>> = vec![vec![0; n]; n];

        let mut offset = n - 1;

        let mut startx = 0;
        let mut starty = 0;

        let mid = n / 2;
        let mut val = 1;
        for _ in 0..mid {
            // let mut x = startx;
            // let mut y = starty;

            for y in starty..(starty + offset) {
                res[startx][y] = val;
                val += 1;
            }

            for x in startx..(startx + offset) {
                res[x][starty + offset] = val;
                val += 1;
            }

            for y in ((starty + 1)..(starty + offset + 1)).rev() {
                res[startx + offset][y] = val;
                val += 1;
            }

            for x in ((startx + 1)..(startx + offset + 1)).rev() {
                res[x][starty] = val;
                val += 1;
            }

            if offset >= 2 {
                offset -= 2;
                startx += 1;
                starty += 1;
            }
        }
        if n % 2 == 1 {
            res[mid][mid] = (n * n) as i32;
        }

        res
    }
}

#[test]
fn leetcode59_t1() {
    // [[1,2,3,4,5],[16,17,18,19,6],[15,24,25,20,7],[14,23,22,21,8],[13,12,11,10,9]]
    let r = Solution::generate_matrix(5);
    println!("{:?}", r);
}
